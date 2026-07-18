mod opts;
use clap::{crate_name, crate_version, Parser};
use env_logger::Env;
use opts::*;
use serde::Serialize;
use ssh_cfg::{SshConfigParser, SshOptionKey, SshSection, SshSectionConfig};
use std::path::{Path, PathBuf};

/// A parsed ssh section together with the config file it was read from.
type SourcedSection = (SshSection, SshSectionConfig, PathBuf);

/// One host entry as emitted by `list --json`.
#[derive(Serialize)]
struct HostRecord {
	host: String,
	source: String,
}

/// Preprocess SSH config content to comment out unsupported options.
///
/// `ssh_cnfg` errors on option keys it does not know about (for instance
/// `UseKeychain`), so we comment those out before parsing.
fn preprocess_ssh_config_content(content: &str) -> String {
	// List of unsupported options to comment out
	let unsupported_options = ["UseKeychain", "IgnoreUnknown"];

	let mut cleaned_content = String::new();
	for line in content.lines() {
		let trimmed = line.trim_start();
		let should_comment = unsupported_options.iter().any(|opt| {
			trimmed.starts_with(opt)
				&& (trimmed.len() == opt.len() || trimmed.chars().nth(opt.len()).is_some_and(char::is_whitespace))
		});

		if should_comment && !trimmed.starts_with('#') {
			cleaned_content.push_str(&format!("# {line}\n"));
			log::debug!("Commented out unsupported option: {trimmed}");
		} else {
			cleaned_content.push_str(line);
			cleaned_content.push('\n');
		}
	}

	cleaned_content
}

/// Resolve an `Include` directive relative to `base_dir`, appending the parsed
/// sections of each referenced file to `sections`.
///
/// An `Include` may list several whitespace separated paths, and each of those
/// may contain glob(7) wildcards (expanded in lexical order), matching
/// OpenSSH's behaviour.
fn resolve_includes(include: &str, base_dir: &Path, sections: &mut Vec<SourcedSection>) {
	for entry in include.split_whitespace() {
		let pattern = base_dir.join(entry);
		let pattern = pattern.to_string_lossy();

		let paths = match glob::glob(&pattern) {
			Ok(paths) => paths,
			Err(error) => {
				log::warn!("Invalid include pattern {pattern}: {error}");
				continue;
			}
		};

		let mut matched: Vec<PathBuf> = paths.filter_map(Result::ok).collect();
		matched.sort();

		if matched.is_empty() {
			log::debug!("Include pattern matched no files: {pattern}");
		}

		for path in matched {
			match load_sections(&path) {
				Ok(mut nested) => sections.append(&mut nested),
				Err(error) => log::warn!("Could not resolve include {}: {error}", path.display()),
			}
		}
	}
}

/// Load the ssh config sections from `path`, resolving `Include` directives
/// relative to the including file's directory. See
/// <https://github.com/chevdor/sshq/issues/1>.
///
/// `ssh_cnfg` surfaces an `Include` that appears before any `Host`/`Match` as a
/// dedicated [`SshSection::Include`], but one that appears inside a
/// `Host`/`Match` block is stored as an [`SshOptionKey::Include`] option within
/// that section. We handle both and splice the included sections in place.
fn load_sections(path: &Path) -> Result<Vec<SourcedSection>, Box<dyn std::error::Error>> {
	let content = std::fs::read_to_string(path)?;
	let cleaned_content = preprocess_ssh_config_content(&content);
	let ssh_config = SshConfigParser::parse_config_contents(&cleaned_content)?;

	let base_dir = path.parent().map_or_else(|| PathBuf::from("."), Path::to_path_buf);

	let mut sections = Vec::new();
	for (section, config) in ssh_config.iter() {
		match section {
			SshSection::Include(include) => resolve_includes(include, &base_dir, &mut sections),
			_ => {
				sections.push((section.clone(), config.clone(), path.to_path_buf()));
				if let Some(include) = config.get(&SshOptionKey::Include) {
					resolve_includes(include, &base_dir, &mut sections);
				}
			}
		}
	}

	Ok(sections)
}

/// Resolve which config file to read: the one given on the command line, or
/// `~/.ssh/config` by default.
fn resolve_config_path(file: Option<&PathBuf>) -> Result<PathBuf, Box<dyn std::error::Error>> {
	match file {
		Some(file) => Ok(file.clone()),
		None => Ok(dirs::home_dir().ok_or("Could not find home directory")?.join(".ssh").join("config")),
	}
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	env_logger::Builder::from_env(Env::default().default_filter_or("none")).init();
	log::debug!("Running {} v{}", crate_name!(), crate_version!());

	let opts: Opts = Opts::parse();
	log::debug!("{opts:#?}");
	let json = opts.json;

	match opts.subcmd {
		SubCommand::Search(search_opts) => {
			log::debug!("search");
			let sections = load_sections(&resolve_config_path(search_opts.file.as_ref())?)?;

			if let Some(pattern) = search_opts.pattern {
				if let Some((SshSection::Host(host_name), host_config, _source)) = sections
					.iter()
					.find(|(section, _, _)| matches!(section, SshSection::Host(h) if h.contains(&pattern)))
				{
					println!("Host: {host_name}");

					for key in [
						SshOptionKey::Hostname,
						SshOptionKey::User,
						SshOptionKey::IdentityFile,
						SshOptionKey::RequestTTY,
						SshOptionKey::IdentitiesOnly,
						SshOptionKey::RemoteForward,
						SshOptionKey::ForwardAgent,
					] {
						if let Some(value) = host_config.get(&key) {
							println!("  {:<15} {value}", key.to_string());
						}
					}
				}
			} else {
				// Print all sections
				println!("{sections:#?}");
			}
		}
		SubCommand::List(list_opts) => {
			log::debug!("list");
			let sections = load_sections(&resolve_config_path(list_opts.file.as_ref())?)?;

			// Flatten `Host` stanzas into individual host names, each tagged with
			// the file it came from (the main config or an included file).
			let records: Vec<HostRecord> = sections
				.iter()
				.filter_map(|(section, _config, source)| match section {
					SshSection::Host(host) => Some((host, source)),
					_ => None,
				})
				.flat_map(|(host, source)| {
					host.split_whitespace()
						.map(move |name| HostRecord { host: name.to_string(), source: source.display().to_string() })
				})
				.collect();

			if json {
				println!("{}", serde_json::to_string_pretty(&records)?);
			} else {
				for record in &records {
					println!("{}", record.host);
				}
			}
		}
	}

	Ok(())
}
