mod opts;
use clap::{crate_name, crate_version, Parser};
use env_logger::Env;
use opts::*;
use ssh_cfg::{SshConfigParser, SshOptionKey};
use tokio::runtime;

/// Preprocess SSH config content to comment out unsupported options
fn preprocess_ssh_config_content(content: &str) -> String {
	// List of unsupported options to comment out
	let unsupported_options = vec!["UseKeychain", "IgnoreUnknown"];

	let mut cleaned_content = String::new();
	for line in content.lines() {
		let trimmed = line.trim_start();
		let should_comment = unsupported_options.iter().any(|opt| {
			trimmed.starts_with(opt)
				&& (trimmed.len() == opt.len() || trimmed.chars().nth(opt.len()).map_or(false, |c| c.is_whitespace()))
		});

		if should_comment && !trimmed.starts_with('#') {
			cleaned_content.push_str(&format!("# {}\n", line));
			log::debug!("Commented out unsupported option: {}", trimmed);
		} else {
			cleaned_content.push_str(line);
			cleaned_content.push('\n');
		}
	}

	cleaned_content
}

async fn parse_ssh_config() -> Result<(), Box<dyn std::error::Error>> {
	env_logger::Builder::from_env(Env::default().default_filter_or("none")).init();
	log::debug!("Running {} v{}", crate_name!(), crate_version!());

	let opts: Opts = Opts::parse();
	log::debug!("{:#?}", opts);

	// Read and preprocess the config file in memory
	let content = if let Some(file) = &opts.file {
		tokio::fs::read_to_string(file).await?
	} else {
		let config_path = dirs::home_dir().ok_or("Could not find home directory")?.join(".ssh").join("config");
		tokio::fs::read_to_string(&config_path).await?
	};

	let cleaned_content = preprocess_ssh_config_content(&content);
	let ssh_config = SshConfigParser::parse_config_contents(&cleaned_content)?;

	match opts.subcmd {
		SubCommand::Search(search_opts) => {
			log::debug!("search");
			let pattern = search_opts.pattern;
			if let Some(p) = pattern {
				if let Some((host_name, host_config)) = ssh_config.iter().find(|c| c.0.contains(&p)) {
					// if let Some((host_name, host_config)) = ssh_config.iter().find(|c| c.0 == &p) {
					println!("Host: {host_name}");

					if let Some(hostname) = host_config.get(&SshOptionKey::Hostname) {
						println!("  {:<15} {}", SshOptionKey::Hostname.to_string(), hostname);
					}

					if let Some(user) = host_config.get(&SshOptionKey::User) {
						println!("  {:<15} {}", SshOptionKey::User.to_string(), user);
					}

					if let Some(identity_file) = host_config.get(&SshOptionKey::IdentityFile) {
						// println!("{}", format!("  {:>30} {}", SshOptionKey::IdentityFile, identity_file));
						println!("  {:<15} {}", SshOptionKey::IdentityFile.to_string(), identity_file);
					}

					if let Some(request_tty) = host_config.get(&SshOptionKey::RequestTTY) {
						println!("  {:<15} {}", SshOptionKey::RequestTTY.to_string(), request_tty);
					}

					if let Some(identities_only) = host_config.get(&SshOptionKey::IdentitiesOnly) {
						println!("  {:<15} {}", SshOptionKey::IdentitiesOnly.to_string(), identities_only);
					}

					if let Some(remote_forward) = host_config.get(&SshOptionKey::RemoteForward) {
						println!("  {:<15} {}", SshOptionKey::RemoteForward.to_string(), remote_forward);
					}

					if let Some(forward_agent) = host_config.get(&SshOptionKey::ForwardAgent) {
						println!("  {:<15} {}", SshOptionKey::ForwardAgent.to_string(), forward_agent);
					}
				}
			} else {
				// Print all host configs
				println!("{ssh_config:#?}");
			}
		}
		SubCommand::List(_list_opts) => {
			log::debug!("list");
			ssh_config.iter().for_each(|(host, _config)| {
				let split = host.split(' ');
				for s in split {
					println!("{s}")
				}
			});
		}
	}

	// // Print first host config

	Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let rt = runtime::Builder::new_current_thread().build()?;
	rt.block_on(parse_ssh_config())
}
