use actix::System;
use futures::future::lazy;
use serde_json::json;
use tempfile::TempDir;

use brace::app::renderer::{Renderer, RendererConfig, Template};
use brace::app::theme::config::ThemeReferenceInfo;

static TEMPLATE_FILE: &'static str = "Hello {{ message }}!";

static THEME_CONF_FILE: &'static str = r#"
[[template]]
name = "custom"
path = "templates/custom.html"
"#;

#[test]
fn test_renderer_tera_template() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    std::fs::create_dir(temp_path.join("templates")).unwrap();
    std::fs::write(temp_path.join("Theme.toml"), THEME_CONF_FILE).unwrap();
    std::fs::write(temp_path.join("templates/custom.html"), TEMPLATE_FILE).unwrap();

    let mut system = System::new("brace_test");
    let config = RendererConfig {
        themes: vec![ThemeReferenceInfo {
            name: Some("custom".to_string()),
            path: temp_path.join("Theme.toml").to_path_buf(),
        }],
    };

    let res = system
        .block_on(lazy(|| {
            Renderer::from_config(config).unwrap().send(Template::new(
                "custom",
                json!({
                    "message": "world"
                }),
            ))
        }))
        .unwrap()
        .unwrap();

    assert_eq!(res, "Hello world!");
}
