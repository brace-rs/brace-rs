use actix::{Handler, Message};
use failure::format_err;
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
    type Result = Result<String, failure::Error>;
}

impl Handler<Template> for RendererInner {
    type Result = Result<String, failure::Error>;

    fn handle(&mut self, msg: Template, _: &mut Self::Context) -> Self::Result {
        match self.0.lock() {
            Ok(res) => match res.render_value(&msg.name, &msg.data) {
                Ok(res) => Ok(res),
                Err(err) => Err(format_err!("{}", err)),
            },
            Err(err) => Err(format_err!("{}", err)),
        }
    }
}
