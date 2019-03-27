use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use assert_cmd::prelude::*;
use tempfile::TempDir;
use walkdir::WalkDir;

static APP_CONF_FILE: &'static str = r#"
[web]
host = "127.0.0.1"
port = 8002

[renderer]
theme = "theme/Theme.toml"
"#;

static THEME_CONF_FILE: &'static str = r#"
[templates]
index = { path = "templates/index.html" }
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
    let app_conf_path = temp_path.join("Config.toml");
    let theme_conf_path = temp_path.join("theme/Theme.toml");
    let template_path = temp_path.join("theme/templates");

    std::fs::create_dir_all(&template_path).unwrap();
    std::fs::write(&app_conf_path, APP_CONF_FILE).unwrap();
    std::fs::write(&theme_conf_path, THEME_CONF_FILE).unwrap();

    for entry in WalkDir::new("theme/templates")
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            let name = path.file_name().unwrap();
            std::fs::copy(path, template_path.join(name)).unwrap();
        }
    }

    let mut process = Command::cargo_bin("brace")
        .unwrap()
        .args(&["web", "run", "--config", app_conf_path.to_str().unwrap()])
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
