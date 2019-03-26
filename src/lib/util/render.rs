use crate::app::config::render::RendererConfig;
use actix::{Actor, Addr, Handler, Message, SyncArbiter, SyncContext};
use serde_json::Value;
use std::ops::Deref;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tera::Tera;

#[derive(Clone)]
pub struct Renderer(pub Addr<RendererInner>);

impl Renderer {
    pub fn new(conf: RendererConfig) -> Self {
        let path = Path::new(&conf.templates).join("**/*");
        let tera = Arc::new(Mutex::new(Tera::new(path.to_str().unwrap()).unwrap()));

        Self(SyncArbiter::start(3, move || RendererInner(tera.clone())))
    }
}

impl Deref for Renderer {
    type Target = Addr<RendererInner>;

    fn deref(&self) -> &Addr<RendererInner> {
        &self.0
    }
}

pub struct RendererInner(pub Arc<Mutex<Tera>>);

impl Actor for RendererInner {
    type Context = SyncContext<Self>;
}

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
