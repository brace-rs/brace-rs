use std::ops::Deref;
use std::path::Path;
use std::sync::{Arc, Mutex};

use actix::{Actor, Addr, SyncArbiter, SyncContext};
use failure::{err_msg, format_err};
use tera::Tera;

pub use self::config::RendererConfig;
pub use self::template::Template;

pub mod config;
pub mod template;

#[derive(Clone)]
pub struct Renderer(pub Addr<RendererInner>);

impl Renderer {
    pub fn from_config(conf: RendererConfig) -> Result<Self, failure::Error> {
        match Path::new(&conf.templates).join("**/*").to_str() {
            Some(path) => match Tera::new(path) {
                Ok(tera) => {
                    let ptr = Arc::new(Mutex::new(tera));

                    Ok(Self(SyncArbiter::start(3, move || {
                        RendererInner(ptr.clone())
                    })))
                }
                Err(err) => Err(format_err!("{}", err)),
            },
            None => Err(err_msg("Invalid template path")),
        }
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
