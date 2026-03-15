use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::str;

struct TestState {
    /// state directory for testing
    dir: PathBuf,
}

impl TestState {
    fn new(name: &str) -> Self {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(".local")
            .join("state")
            .join(name);
        fs::create_dir_all(&dir).unwrap();
        TestState { dir }
    }

    fn stok(&self) -> Command {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_stok"));

        {
            #[cfg(unix)]
            cmd.env("XDG_STATE_HOME", &self.dir);
            #[cfg(windows)]
            cmd.env("LOCALAPPDATA", &self.dir);
        }

        cmd
    }
}

impl Drop for TestState {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.dir);
    }
}

#[test]
fn test_set_and_run() {
    let state = TestState::new("test_set_and_run");

    // Store two keys
    let status = state
        .stok()
        .args(["--set", "MY_TOKEN", "--value", "tk-secret"])
        .status()
        .unwrap();
    assert!(status.success());

    let status = state
        .stok()
        .args(["--set", "OTHER_KEY", "--value", "other-val"])
        .status()
        .unwrap();
    assert!(status.success());

    // Run printenv with stok: reference in env; resolved value should be printed
    let output = state
        .stok()
        .env("MY_TOKEN", "stok:MY_TOKEN")
        .args(["printenv", "MY_TOKEN"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(str::from_utf8(&output.stdout).unwrap().trim(), "tk-secret");
}

#[test]
fn test_unset() {
    let state = TestState::new("test_unset");

    state
        .stok()
        .args(["--set", "DEL_KEY", "--value", "to-delete"])
        .status()
        .unwrap();
    let status = state.stok().args(["--unset", "DEL_KEY"]).status().unwrap();
    assert!(status.success());

    // Resolving the unset key should fail
    let output = state
        .stok()
        .env("DEL_KEY", "stok:DEL_KEY")
        .args(["printenv", "DEL_KEY"])
        .output()
        .unwrap();
    assert!(!output.status.success());
}

#[test]
fn test_ls() {
    let state = TestState::new("test_ls");

    for (k, v) in [
        ("API_TOKEN", "v1"),
        ("DB_PASSWORD", "v2"),
        ("API_SECRET", "v3"),
    ] {
        state
            .stok()
            .args(["--set", k, "--value", v])
            .status()
            .unwrap();
    }

    // List all
    let output = state.stok().args(["--ls"]).output().unwrap();
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("API_TOKEN"));
    assert!(stdout.contains("DB_PASSWORD"));
    assert!(stdout.contains("API_SECRET"));
}

#[test]
fn test_unknown_option_error() {
    let state = TestState::new("test_unknown_option_error");

    let output = state.stok().args(["-unknown"]).output().unwrap();
    assert!(!output.status.success());
    assert!(str::from_utf8(&output.stderr)
        .unwrap()
        .contains("unknown option"));
}

#[test]
fn test_env_file_only_loads_stok_references() {
    let state = TestState::new("test_env_file_only_loads_stok_references");

    state
        .stok()
        .args(["--set", "SECRET", "--value", "resolved-value"])
        .status()
        .unwrap();

    // Write a .env file with one stok: reference and one plain value
    let env_file = state.dir.join("test.env");
    let mut f = File::create(&env_file).unwrap();
    writeln!(
        f,
        r#"FROM_STOK=stok:SECRET
PLAIN_VAR=plain-value
"#
    )
    .unwrap();

    // FROM_STOK should be resolved; PLAIN_VAR should NOT be injected
    let output = state
        .stok()
        .args([
            "--env-file",
            env_file.to_str().unwrap(),
            "printenv",
            "FROM_STOK",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(
        str::from_utf8(&output.stdout).unwrap().trim(),
        "resolved-value"
    );

    let output = state
        .stok()
        .args([
            "--env-file",
            env_file.to_str().unwrap(),
            "printenv",
            "PLAIN_VAR",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
}
