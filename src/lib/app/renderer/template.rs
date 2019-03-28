use actix::{Handler, Message};
use failure::Error;
use serde_json::Value;

use super::RendererInner;

pub struct Template {
    pub name: String,
    pub data: Value,
}

impl Template {
    pub fn new<S: Into<String>>(name: S, data: Value) -> Self {
        Self {
            name: name.into(),
            data,
        }
    }
}

impl Message for Template {
    type Result = Result<String, Error>;
}

impl Handler<Template> for RendererInner {
    type Result = Result<String, Error>;

    fn handle(&mut self, msg: Template, _: &mut Self::Context) -> Self::Result {
        self.render_template(msg.name, msg.data)
    }
}
