use actix::System;
use brace::config::render::RendererConfig;
use brace::util::render::{Renderer, Template};
use futures::future::lazy;
use serde_json::json;
use std::fs::File;
use std::io::Write;
use tempfile::TempDir;

static TEMPLATE_FILE: &'static str = "Hello {{ message }}!";

#[test]
fn test_renderer_tera_template() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path().as_os_str().to_str().unwrap();
    let file_path = temp_dir.path().join("template.html");
    let mut temp_file = File::create(file_path).unwrap();

    writeln!(temp_file, "{}", TEMPLATE_FILE).unwrap();

    let mut system = System::new("brace_test");
    let config = RendererConfig {
        templates: temp_path.to_string(),
    };

    let res = system
        .block_on(lazy(|| {
            Renderer::new(config).send(Template::new(
                "template.html".into(),
                json!({
                    "message": "world"
                }),
            ))
        }))
        .unwrap()
        .unwrap();

    assert_eq!(res, "Hello world!\n")
}
