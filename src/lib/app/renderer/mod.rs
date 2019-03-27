use std::ops::Deref;
use std::sync::{Arc, Mutex};

use actix::{Actor, Addr, SyncArbiter, SyncContext};
use failure::format_err;
use path_absolutize::Absolutize;
use tera::Tera;

use super::theme::ThemeConfig;

pub use self::config::RendererConfig;
pub use self::template::Template;

pub mod config;
pub mod template;

#[derive(Clone)]
pub struct Renderer(pub Addr<RendererInner>);

impl Renderer {
    pub fn from_config(conf: RendererConfig) -> Result<Self, failure::Error> {
        let theme_path = conf.theme;
        let theme_conf = ThemeConfig::from_file(&theme_path)?;

        match theme_path.parent() {
            Some(parent) => {
                let mut tera = Tera::default();
                let files = (&theme_conf.templates)
                    .iter()
                    .map(|(key, template)| {
                        (
                            parent.join(template.path.clone()).absolutize().unwrap(),
                            Some(key.as_ref()),
                        )
                    })
                    .collect();

                match tera.add_template_files(files) {
                    Ok(_) => {
                        let ptr = Arc::new(Mutex::new(tera));

                        Ok(Self(SyncArbiter::start(3, move || {
                            RendererInner(ptr.clone())
                        })))
                    }
                    Err(err) => Err(format_err!("{}", err)),
                }
            }
            None => Err(format_err!("Invalid theme path {:?}", theme_path)),
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
