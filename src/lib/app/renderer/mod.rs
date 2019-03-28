use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use actix::{Actor, Addr, SyncArbiter, SyncContext};
use failure::{format_err, Error};
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
    pub fn from_config(conf: RendererConfig) -> Result<Self, Error> {
        let mut tera = Tera::default();

        for theme in conf.themes {
            let path = theme.path;
            let conf = ThemeConfig::from_file(&path)?;

            match path.parent() {
                Some(path) => {
                    if let Err(err) = Self::add_template_files(&mut tera, path, &conf) {
                        return Err(err);
                    }
                }
                None => return Err(format_err!("Invalid theme path {:?}", path)),
            }
        }

        let ptr = Arc::new(Mutex::new(tera));

        Ok(Self(SyncArbiter::start(3, move || {
            RendererInner(ptr.clone())
        })))
    }

    fn add_template_files<'a>(
        tera: &mut Tera,
        path: &Path,
        conf: &'a ThemeConfig,
    ) -> Result<(), Error> {
        match tera.add_template_files(Self::get_template_files(path, &conf)) {
            Ok(_) => Ok(()),
            Err(err) => Err(format_err!("{}", err)),
        }
    }

    fn get_template_files<'a>(
        path: &Path,
        conf: &'a ThemeConfig,
    ) -> Vec<(PathBuf, Option<&'a str>)> {
        conf.templates
            .iter()
            .map(|template| {
                (
                    path.join(template.path.clone()).absolutize().unwrap(),
                    Some(template.name.as_ref()),
                )
            })
            .collect()
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
