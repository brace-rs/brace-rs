use std::process::Command;

use assert_cmd::prelude::*;
use tempfile::TempDir;

static CMD_INVALID_ERR: &'static str = "\
error: Invalid subcommand: invalid

USAGE:
    brace [FLAGS] [OPTIONS] [SUBCOMMAND]
";

#[test]
fn test_command_invalid() {
    Command::cargo_bin("brace")
        .unwrap()
        .arg("invalid")
        .assert()
        .failure()
        .code(1)
        .stderr(CMD_INVALID_ERR);
}

static CMD_MISSING_ERR: &'static str = "\
error: Expected a valid subcommand

USAGE:
    brace [FLAGS] [OPTIONS] [SUBCOMMAND]
";

#[test]
fn test_command_missing() {
    Command::cargo_bin("brace")
        .unwrap()
        .assert()
        .failure()
        .code(1)
        .stderr(CMD_MISSING_ERR);
}

#[test]
fn test_command_quiet() {
    Command::cargo_bin("brace")
        .unwrap()
        .arg("--quiet")
        .assert()
        .failure()
        .code(1)
        .stderr("");
}

static CMD_INIT_OUT: &'static str = "\
info: Created new site at {}
";

#[test]
fn test_command_init() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().as_os_str().to_str().unwrap();

    Command::cargo_bin("brace")
        .unwrap()
        .args(&["-v", "init", path])
        .assert()
        .success()
        .stderr(CMD_INIT_OUT.replace("{}", path));
}

static CMD_WEB_INVALID_ERR: &'static str = "\
error: Invalid subcommand: invalid

USAGE:
    brace web [SUBCOMMAND]
";

#[test]
fn test_command_web_invalid() {
    Command::cargo_bin("brace")
        .unwrap()
        .args(&["web", "invalid"])
        .assert()
        .failure()
        .code(1)
        .stderr(CMD_WEB_INVALID_ERR);
}

static CMD_WEB_MISSING_ERR: &'static str = "\
error: Expected a valid subcommand

USAGE:
    brace web [SUBCOMMAND]
";

#[test]
fn test_command_web_missing() {
    Command::cargo_bin("brace")
        .unwrap()
        .arg("web")
        .assert()
        .failure()
        .code(1)
        .stderr(CMD_WEB_MISSING_ERR);
}
