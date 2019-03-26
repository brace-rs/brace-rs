use super::RendererInner;
use actix::{Handler, Message};
use serde_json::Value;

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
                Err(_) => Err(failure::format_err!(
                    "Failed to render template {}",
                    msg.name
                )),
            },
            Err(_) => Err(failure::format_err!(
                "Failed to render template {}",
                msg.name
            )),
        }
    }
}
