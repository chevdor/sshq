use assert_cmd::Command;
use std::env;
use std::str;

fn get_command() -> Command {
	// // TODO is there no cleaner way to get this from Cargo?
	// // Also should it really be "debug"?
	// let target_dir: PathBuf = env::var_os("CARGO_TARGET_DIR").unwrap_or_else(|| OsString::from("target")).into();
	// target_dir.join("debug").join("sshq")
	Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Failed getting test bin")
}

#[test]
fn help_if_no_args() {
	// Probably want to factor out much of this when adding more tests.
	let proc = get_command().output().unwrap();
	assert!(!proc.status.success());
	let stderr = str::from_utf8(proc.stderr.as_slice()).unwrap();
	assert!(stderr.contains("-h, --help"));
}

#[test]
fn list() {
	let mut cmd = get_command();
	env::set_var("RUST_LOG", "none");

	let assert = cmd.args(["list", "tests/data/config"]).assert();
	let foo = assert.success().code(0);
	let out: Vec<String> = String::from_utf8(foo.get_output().stdout.to_vec())
		.unwrap()
		.split('\n')
		.map(ToString::to_string)
		.filter(|s| !s.is_empty())
		.collect();
	// `*` and `foo` from the main config, plus `bar` pulled in via the `Include` directive.
	assert_eq!(3, out.len());

	println!("out = {:?}", out);
}

#[test]
fn list_resolves_include() {
	let mut cmd = get_command();
	env::set_var("RUST_LOG", "none");

	let assert = cmd.args(["list", "tests/data/config"]).assert();
	let output = assert.success().code(0);
	let hosts: Vec<String> = String::from_utf8(output.get_output().stdout.to_vec())
		.unwrap()
		.lines()
		.filter(|s| !s.is_empty())
		.map(ToString::to_string)
		.collect();

	// `bar` is only defined in tests/data/config.d/sample and reachable solely
	// through the `Include config.d/sample` directive in tests/data/config.
	assert!(hosts.iter().any(|h| h == "bar"), "expected included host `bar`, got {hosts:?}");
	assert!(hosts.iter().any(|h| h == "foo"));
	assert!(hosts.iter().any(|h| h == "*"));
}

#[test]
fn search_finds_included_host() {
	let mut cmd = get_command();
	env::set_var("RUST_LOG", "none");

	let assert = cmd.args(["search", "bar", "tests/data/config"]).assert();
	let output = assert.success().code(0);
	let stdout = String::from_utf8(output.get_output().stdout.to_vec()).unwrap();

	// `bar` comes from the included file; searching for it must surface its config.
	assert!(stdout.contains("Host: bar"), "expected `Host: bar`, got:\n{stdout}");
	assert!(stdout.contains("192.168.1.3"), "expected bar's Hostname, got:\n{stdout}");
}
