use actix::System;
use futures::future::lazy;
use serde_json::json;
use tempfile::TempDir;

use brace::app::renderer::{Renderer, RendererConfig, Template};

static TEMPLATE_FILE: &'static str = "Hello {{ message }}!";

static THEME_CONF_FILE: &'static str = r#"
[templates]
custom = { path = "templates/custom.html" }
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
        theme: temp_path.join("Theme.toml").to_path_buf(),
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
