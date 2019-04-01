use std::process::Command;

use actix::System;
use assert_cmd::prelude::*;
use futures::future::lazy;
use serde_json::{json, Value};
use tempfile::TempDir;

use brace_theme::config::ThemeReferenceInfo;
use brace_theme::renderer::{Renderer, RendererConfig, Template};

static TEMPLATE_FILE: &'static str = "Hello {{ message }}!";

static TEMPLATE_FILE_FN: &'static str = r#"
I said {{ template(name="custom-tera", value=map(key="message", value=message)) }}
"#;

static THEME_CONF_FILE: &'static str = r#"
[theme]
name = "custom"
label = "Custom"

[[manifests]]
path = "manifest.toml"
"#;

static THEME_MANF_FILE: &'static str = r#"
[manifest]
name = "custom"
label = "Custom"

[[templates]]
name = "custom-static"
type = "static"
path = "templates/custom-static.html"

[[templates]]
name = "custom-tera"
type = "tera"
path = "templates/custom-tera.html"

[[templates]]
name = "custom-tera-fn"
type = "tera"
path = "templates/custom-tera-fn.html"

[[templates]]
name = "custom-text"
type = "text"
text = "Hello {{ message }}!"
"#;

#[test]
fn test_theme_command_init() {
    let dir = TempDir::new().unwrap();
    let path = dir.path();

    Command::cargo_bin("brace")
        .unwrap()
        .args(&["theme", "init", path.to_str().unwrap()])
        .assert()
        .success();

    let cfg = std::fs::metadata(path.join("theme.toml")).unwrap();
    let man = std::fs::metadata(path.join("manifest.toml")).unwrap();
    let tpl = std::fs::metadata(path.join("templates/index.html")).unwrap();

    assert!(cfg.is_file());
    assert!(man.is_file());
    assert!(tpl.is_file());
}

#[test]
fn test_theme_template_render_static() {
    let dir = TempDir::new().unwrap();
    let path = dir.path();

    std::fs::create_dir(path.join("templates")).unwrap();
    std::fs::write(path.join("theme.toml"), THEME_CONF_FILE).unwrap();
    std::fs::write(path.join("manifest.toml"), THEME_MANF_FILE).unwrap();
    std::fs::write(path.join("templates/custom-static.html"), TEMPLATE_FILE).unwrap();
    std::fs::write(path.join("templates/custom-tera.html"), TEMPLATE_FILE).unwrap();
    std::fs::write(path.join("templates/custom-tera-fn.html"), TEMPLATE_FILE_FN).unwrap();

    let mut system = System::new("brace_test");
    let config = RendererConfig {
        themes: vec![ThemeReferenceInfo {
            name: Some("custom".to_string()),
            path: path.join("theme.toml").to_path_buf(),
        }],
    };

    let res = system
        .block_on(lazy(|| {
            Renderer::from_config(config)
                .unwrap()
                .send(Template::new("custom-static", Value::Null))
        }))
        .unwrap()
        .unwrap();

    assert_eq!(res, "Hello {{ message }}!");
}

#[test]
fn test_theme_template_render_tera() {
    let dir = TempDir::new().unwrap();
    let path = dir.path();

    std::fs::create_dir(path.join("templates")).unwrap();
    std::fs::write(path.join("theme.toml"), THEME_CONF_FILE).unwrap();
    std::fs::write(path.join("manifest.toml"), THEME_MANF_FILE).unwrap();
    std::fs::write(path.join("templates/custom-static.html"), TEMPLATE_FILE).unwrap();
    std::fs::write(path.join("templates/custom-tera.html"), TEMPLATE_FILE).unwrap();
    std::fs::write(path.join("templates/custom-tera-fn.html"), TEMPLATE_FILE_FN).unwrap();

    let mut system = System::new("brace_test");
    let config = RendererConfig {
        themes: vec![ThemeReferenceInfo {
            name: Some("custom".to_string()),
            path: path.join("theme.toml").to_path_buf(),
        }],
    };

    let res = system
        .block_on(lazy(|| {
            Renderer::from_config(config).unwrap().send(Template::new(
                "custom-tera",
                json!({
                    "message": "world"
                }),
            ))
        }))
        .unwrap()
        .unwrap();

    assert_eq!(res, "Hello world!");
}

#[test]
fn test_theme_template_render_text() {
    let dir = TempDir::new().unwrap();
    let path = dir.path();

    std::fs::create_dir(path.join("templates")).unwrap();
    std::fs::write(path.join("theme.toml"), THEME_CONF_FILE).unwrap();
    std::fs::write(path.join("manifest.toml"), THEME_MANF_FILE).unwrap();
    std::fs::write(path.join("templates/custom-static.html"), TEMPLATE_FILE).unwrap();
    std::fs::write(path.join("templates/custom-tera.html"), TEMPLATE_FILE).unwrap();
    std::fs::write(path.join("templates/custom-tera-fn.html"), TEMPLATE_FILE_FN).unwrap();

    let mut system = System::new("brace_test");
    let config = RendererConfig {
        themes: vec![ThemeReferenceInfo {
            name: Some("custom".to_string()),
            path: path.join("theme.toml").to_path_buf(),
        }],
    };

    let res = system
        .block_on(lazy(|| {
            Renderer::from_config(config)
                .unwrap()
                .send(Template::new("custom-text", Value::Null))
        }))
        .unwrap()
        .unwrap();

    assert_eq!(res, "Hello {{ message }}!");
}

#[test]
fn test_theme_template_render_fn() {
    let dir = TempDir::new().unwrap();
    let path = dir.path();

    std::fs::create_dir(path.join("templates")).unwrap();
    std::fs::write(path.join("theme.toml"), THEME_CONF_FILE).unwrap();
    std::fs::write(path.join("manifest.toml"), THEME_MANF_FILE).unwrap();
    std::fs::write(path.join("templates/custom-static.html"), TEMPLATE_FILE).unwrap();
    std::fs::write(path.join("templates/custom-tera.html"), TEMPLATE_FILE).unwrap();
    std::fs::write(path.join("templates/custom-tera-fn.html"), TEMPLATE_FILE_FN).unwrap();

    let mut system = System::new("brace_test");
    let config = RendererConfig {
        themes: vec![ThemeReferenceInfo {
            name: Some("custom".to_string()),
            path: path.join("theme.toml").to_path_buf(),
        }],
    };

    let res = system
        .block_on(lazy(|| {
            Renderer::from_config(config).unwrap().send(Template::new(
                "custom-tera-fn",
                json!({
                    "message": "universe"
                }),
            ))
        }))
        .unwrap()
        .unwrap();

    assert_eq!(res, "\nI said Hello universe!\n");
}
