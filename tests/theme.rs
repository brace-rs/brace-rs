use std::process::Command;

use assert_cmd::prelude::*;
use tempfile::TempDir;

#[test]
fn test_command_theme_init() {
    let dir = TempDir::new().unwrap();
    let path = dir.path();

    Command::cargo_bin("brace")
        .unwrap()
        .args(&["theme", "init", path.to_str().unwrap()])
        .assert()
        .success();

    let cfg = std::fs::metadata(path.join("Theme.toml")).unwrap();
    let tpl = std::fs::metadata(path.join("templates/index.html")).unwrap();

    assert!(cfg.is_file());
    assert!(tpl.is_file());
}
