mod opts;
use clap::{crate_name, crate_version, Parser};
use env_logger::Env;
use opts::*;
use ssh_cfg::{SshConfigParser, SshOptionKey};
use tokio::runtime;

async fn parse_ssh_config() -> Result<(), Box<dyn std::error::Error>> {
	env_logger::Builder::from_env(Env::default().default_filter_or("none")).init();
	log::info!("Running {} v{}", crate_name!(), crate_version!());

	let opts: Opts = Opts::parse();
	let ssh_config = SshConfigParser::parse_home().await?;

	match opts.subcmd {
		SubCommand::Search(search_opts) => {
			log::info!("search");
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
			log::info!("list");
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
