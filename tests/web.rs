use assert_cmd::prelude::*;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use tempfile::TempDir;

static CONFIG_FILE: &'static str = r#"
[web]
host = "127.0.0.1"
port = 8002
"#;

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
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    brace::app::init::init(temp_path.to_str().unwrap()).unwrap();

    let conf_path = temp_path.join("Config.toml");
    let mut conf_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&conf_path)
        .unwrap();

    write!(conf_file, "{}", CONFIG_FILE).unwrap();

    let mut process = Command::cargo_bin("brace")
        .unwrap()
        .args(&["web", "run", "--config", conf_path.to_str().unwrap()])
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
