pub use self::config::RendererConfig;
pub use self::template::Template;
use actix::{Actor, Addr, SyncArbiter, SyncContext};

use std::ops::Deref;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tera::Tera;

pub mod config;
pub mod template;

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
