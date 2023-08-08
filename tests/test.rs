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
	// It will be 2 until Include is supported, it will then be 3 with the example we have
	assert_eq!(2, out.len());

	println!("out = {:?}", out);
}
