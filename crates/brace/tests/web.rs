use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use assert_cmd::prelude::*;
use brace::config::AppConfig;
use tempfile::TempDir;

#[test]
fn test_web_server_without_config() {
    let mut process = Command::cargo_bin("brace")
        .unwrap()
        .current_dir("../..")
        .args(&["web", "run"])
        .spawn()
        .unwrap();

    sleep(Duration::from_millis(500));

    let res1 = reqwest::get("http://127.0.0.1:8080");
    let res2 = reqwest::get("http://127.0.0.1:8080/themes");
    let res3 = reqwest::get("http://127.0.0.1:8080/404");

    process.kill().unwrap();

    assert_eq!(res1.unwrap().status(), 200);
    assert_eq!(res2.unwrap().status(), 200);
    assert_eq!(res3.unwrap().status(), 404);
}

#[test]
fn test_web_server_with_arguments() {
    let mut process = Command::cargo_bin("brace")
        .unwrap()
        .current_dir("../..")
        .args(&["web", "run", "--host", "127.0.0.1", "--port", "8001"])
        .spawn()
        .unwrap();

    sleep(Duration::from_millis(500));

    let res1 = reqwest::get("http://127.0.0.1:8001");
    let res2 = reqwest::get("http://127.0.0.1:8001/themes");
    let res3 = reqwest::get("http://127.0.0.1:8001/404");

    process.kill().unwrap();

    assert_eq!(res1.unwrap().status(), 200);
    assert_eq!(res2.unwrap().status(), 200);
    assert_eq!(res3.unwrap().status(), 404);
}

#[test]
fn test_web_server_with_config() {
    let dir = TempDir::new().unwrap();
    let path = dir.path();
    let mut config = AppConfig::default();

    config.web.port = 8002;

    brace::init(config, path).unwrap();

    let mut process = Command::cargo_bin("brace")
        .unwrap()
        .args(&[
            "web",
            "run",
            "--config",
            path.join("config.toml").to_str().unwrap(),
        ])
        .spawn()
        .unwrap();

    sleep(Duration::from_millis(500));

    let res1 = reqwest::get("http://127.0.0.1:8002");
    let res2 = reqwest::get("http://127.0.0.1:8002/themes");
    let res3 = reqwest::get("http://127.0.0.1:8002/404");

    process.kill().unwrap();

    assert_eq!(res1.unwrap().status(), 200);
    assert_eq!(res2.unwrap().status(), 500);
    assert_eq!(res3.unwrap().status(), 404);
}
