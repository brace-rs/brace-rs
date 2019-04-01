use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use tera::{Error as TeraError, Function, Result as TeraResult, Tera};

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum TemplateInfo {
    Static { name: String, path: PathBuf },
    Tera { name: String, path: PathBuf },
    Text { name: String, text: String },
}

impl TemplateInfo {
    pub fn name(&self) -> &String {
        match self {
            TemplateInfo::Static { name, .. } => name,
            TemplateInfo::Tera { name, .. } => name,
            TemplateInfo::Text { name, .. } => name,
        }
    }
}

pub struct TemplateFunction {
    pub tera: Arc<RwLock<Tera>>,
    pub templates: HashMap<String, TemplateInfo>,
}

impl TemplateFunction {
    pub fn render_template(&self, name: &str, value: &Value) -> TeraResult<Value> {
        match self.templates.get(name) {
            Some(info) => match value {
                Value::Object(_) => match info {
                    TemplateInfo::Static { path, .. } => match read_to_string(path) {
                        Ok(str) => Ok(Value::String(str)),
                        Err(err) => Err(TeraError::msg(format!("{}", err))),
                    },
                    TemplateInfo::Text { text, .. } => Ok(Value::String(text.to_string())),
                    TemplateInfo::Tera { name, .. } => match self.tera.read() {
                        Ok(tera) => match tera.render_value(&name, &value) {
                            Ok(res) => Ok(Value::String(res)),
                            Err(err) => Err(err),
                        },
                        Err(err) => Err(TeraError::msg(format!("{}", err))),
                    },
                },
                _ => Err(TeraError::msg(format!(
                    "Global function `template` received value={} but `value` can only be an object",
                    value
                ))),
            },
            None => Err(TeraError::msg(format!(
                "Global function `template` received name={} but `name` is not a valid template",
                name
            ))),
        }
    }
}

impl Function for TemplateFunction {
    fn call(&self, args: &HashMap<String, Value>) -> TeraResult<Value> {
        match args.get("name") {
            Some(name) => match args.get("value") {
                Some(value) => match name {
                    Value::String(name) => self.render_template(name, value),
                    _ => Err(TeraError::msg(format!(
                        "Global function `template` received name={} but `name` can only be a string",
                        name
                    ))),
                },
                None => Err(TeraError::msg(
                    "Global function `template` was called without a `value` argument",
                )),
            },
            None => Err(TeraError::msg(
                "Global function `template` was called without a `name` argument",
            )),
        }
    }
}

pub struct MapFunction;

impl Function for MapFunction {
    fn call(&self, args: &HashMap<String, Value>) -> TeraResult<Value> {
        match args.get("key") {
            Some(key) => match key {
                Value::String(key) => match args.get("value") {
                    Some(value) => {
                        let mut map = Map::new();

                        map.insert(key.to_string(), value.clone());

                        Ok(Value::Object(map))
                    }
                    None => Err(TeraError::msg(
                        "Global function `map` was called without a `value` argument",
                    )),
                },
                _ => Err(TeraError::msg(format!(
                    "Global function `map` received key={} but `key` can only be a string",
                    key
                ))),
            },
            None => Err(TeraError::msg(
                "Global function `map` was called without a `key` argument",
            )),
        }
    }
}
