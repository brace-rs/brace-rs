use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use assert_cmd::prelude::*;
use tempfile::TempDir;

use brace::app::AppConfig;

#[test]
fn test_web_server_without_config() {
    let mut process = Command::cargo_bin("brace")
        .unwrap()
        .args(&["web", "run"])
        .spawn()
        .unwrap();

    sleep(Duration::from_millis(200));

    let res = reqwest::get("http://127.0.0.1:8080");

    process.kill().unwrap();

    assert_eq!(res.unwrap().status(), 200);
}

#[test]
fn test_web_server_with_arguments() {
    let mut process = Command::cargo_bin("brace")
        .unwrap()
        .args(&["web", "run", "--host", "127.0.0.1", "--port", "8001"])
        .spawn()
        .unwrap();

    sleep(Duration::from_millis(200));

    let res = reqwest::get("http://127.0.0.1:8001");

    process.kill().unwrap();

    assert_eq!(res.unwrap().status(), 200);
}

#[test]
fn test_web_server_with_config() {
    let dir = TempDir::new().unwrap();
    let path = dir.path();
    let mut config = AppConfig::default();

    config.web.port = 8002;

    brace::app::init(config, path).unwrap();

    let mut process = Command::cargo_bin("brace")
        .unwrap()
        .args(&[
            "web",
            "run",
            "--config",
            path.join("Config.toml").to_str().unwrap(),
        ])
        .spawn()
        .unwrap();

    sleep(Duration::from_millis(200));

    let res = reqwest::get("http://127.0.0.1:8002");

    process.kill().unwrap();

    assert_eq!(res.unwrap().status(), 200);
}

#[test]
fn test_web_server_404() {
    let mut process = Command::cargo_bin("brace")
        .unwrap()
        .args(&["web", "run", "--host", "127.0.0.1", "--port", "8003"])
        .spawn()
        .unwrap();

    sleep(Duration::from_millis(200));

    let res = reqwest::get("http://127.0.0.1:8003/404");

    process.kill().unwrap();

    assert_eq!(res.unwrap().status(), 404);
}
