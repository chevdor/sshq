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
fn list_resolves_glob_include() {
	let mut cmd = get_command();
	env::set_var("RUST_LOG", "none");

	// tests/data/config_glob pulls in tests/data/config.d/* via a glob Include.
	let assert = cmd.args(["list", "tests/data/config_glob"]).assert();
	let output = assert.success().code(0);
	let hosts: Vec<String> = String::from_utf8(output.get_output().stdout.to_vec())
		.unwrap()
		.lines()
		.filter(|s| !s.is_empty())
		.map(ToString::to_string)
		.collect();

	// `alpha` from the main file, `bar` reached through the `Include config.d/*` glob.
	assert!(hosts.iter().any(|h| h == "alpha"), "expected `alpha`, got {hosts:?}");
	assert!(hosts.iter().any(|h| h == "bar"), "expected glob-included host `bar`, got {hosts:?}");
}

#[test]
fn list_json_outputs_valid_json() {
	let mut cmd = get_command();
	env::set_var("RUST_LOG", "none");

	// Plain `list` prints one host per line; `list -j` must emit JSON instead.
	let plain = String::from_utf8(get_command().args(["list", "tests/data/config"]).output().unwrap().stdout).unwrap();

	let assert = cmd.args(["list", "-j", "tests/data/config"]).assert();
	let output = assert.success().code(0);
	let stdout = String::from_utf8(output.get_output().stdout.to_vec()).unwrap();

	assert_ne!(plain, stdout, "`-j` output should differ from the plain listing");

	let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("`list -j` must produce valid JSON");
	assert!(parsed.is_array(), "expected a JSON array, got: {stdout}");
}

#[test]
fn list_json_includes_source() {
	let mut cmd = get_command();
	env::set_var("RUST_LOG", "none");

	let assert = cmd.args(["list", "-j", "tests/data/config"]).assert();
	let output = assert.success().code(0);
	let stdout = String::from_utf8(output.get_output().stdout.to_vec()).unwrap();

	let entries: Vec<serde_json::Value> = serde_json::from_str(&stdout).unwrap();

	// Every entry carries both a host and its source file.
	assert!(
		entries
			.iter()
			.all(|e| e.get("host").and_then(|h| h.as_str()).is_some()
				&& e.get("source").and_then(|s| s.as_str()).is_some()),
		"every entry needs host + source, got: {stdout}"
	);

	// `bar` is only defined in the included file, so its source must point there.
	let bar = entries.iter().find(|e| e["host"] == "bar").expect("included host `bar` present");
	assert!(
		bar["source"].as_str().unwrap().contains("config.d/sample"),
		"bar's source should be the included file, got {}",
		bar["source"]
	);

	// `foo` comes from the main config, not the included file.
	let foo = entries.iter().find(|e| e["host"] == "foo").expect("host `foo` present");
	let foo_source = foo["source"].as_str().unwrap();
	assert!(
		foo_source.contains("tests/data/config") && !foo_source.contains("config.d/sample"),
		"foo source: {foo_source}"
	);
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
