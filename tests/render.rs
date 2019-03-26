use std::fs::File;
use std::io::Write;

use actix::System;
use futures::future::lazy;
use serde_json::json;
use tempfile::TempDir;

use brace::app::renderer::{Renderer, RendererConfig, Template};

static TEMPLATE_FILE: &'static str = "Hello {{ message }}!";

#[test]
fn test_renderer_tera_template() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path().to_str().unwrap();
    let file_path = temp_dir.path().join("template.html");
    let mut temp_file = File::create(file_path).unwrap();

    writeln!(temp_file, "{}", TEMPLATE_FILE).unwrap();

    let mut system = System::new("brace_test");
    let config = RendererConfig {
        templates: temp_path.to_string(),
    };

    let res = system
        .block_on(lazy(|| {
            Renderer::from_config(config).send(Template::new(
                "template.html",
                json!({
                    "message": "world"
                }),
            ))
        }))
        .unwrap()
        .unwrap();

    assert_eq!(res, "Hello world!\n")
}
