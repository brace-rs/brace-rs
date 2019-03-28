use std::collections::HashMap;
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use actix::{Actor, Addr, SyncArbiter, SyncContext};
use failure::{format_err, Error};
use path_absolutize::Absolutize;
use serde_json::Value;
use tera::Tera;

use super::theme::template::TemplateInfo;
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
        let mut templates = HashMap::new();

        for theme in conf.themes {
            let path = theme.path;
            let mut conf = ThemeConfig::from_file(&path)?;

            match path.parent() {
                Some(dir) => {
                    for template in conf.templates.iter_mut() {
                        match template {
                            TemplateInfo::Static { ref mut path, .. } => {
                                *path = dir.join(&path).absolutize()?;
                            }
                            TemplateInfo::Tera { ref mut path, .. } => {
                                *path = dir.join(&path).absolutize()?;
                            }
                            TemplateInfo::Text { .. } => (),
                        }

                        templates.insert(template.name().to_owned(), template.clone());
                    }
                }
                None => return Err(format_err!("Invalid theme path {:?}", path)),
            }

            if let Err(err) = Self::add_template_files(&mut tera, &conf) {
                return Err(err);
            }
        }

        let ptr = Arc::new(Mutex::new(tera));

        Ok(Self(SyncArbiter::start(3, move || RendererInner {
            tera: ptr.clone(),
            templates: templates.clone(),
        })))
    }

    fn add_template_files<'a>(tera: &mut Tera, conf: &'a ThemeConfig) -> Result<(), Error> {
        match tera.add_template_files(Self::get_template_files(&conf)) {
            Ok(_) => Ok(()),
            Err(err) => Err(format_err!("{}", err)),
        }
    }

    fn get_template_files<'a>(conf: &'a ThemeConfig) -> Vec<(PathBuf, Option<&'a str>)> {
        conf.templates
            .iter()
            .filter_map(|template| match template {
                TemplateInfo::Tera { name, path } => Some((path.clone(), Some(name.as_ref()))),
                _ => None,
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

pub struct RendererInner {
    tera: Arc<Mutex<Tera>>,
    templates: HashMap<String, TemplateInfo>,
}

impl RendererInner {
    pub fn render_template<K, V>(&self, name: K, data: V) -> Result<String, Error>
    where
        K: Into<String>,
        V: Into<Value>,
    {
        let name = name.into();

        match self.templates.get(&name) {
            Some(info) => match info {
                TemplateInfo::Static { path, .. } => Ok(std::fs::read_to_string(path)?),
                TemplateInfo::Text { text, .. } => Ok(text.to_string()),
                TemplateInfo::Tera { name, .. } => match self.tera.lock() {
                    Ok(res) => match res.render_value(&name, &data.into()) {
                        Ok(res) => Ok(res),
                        Err(err) => Err(format_err!("{}", err)),
                    },
                    Err(err) => Err(format_err!("{}", err)),
                },
            },
            None => Err(format_err!("Template '{}' does not exist", &name)),
        }
    }
}

impl Actor for RendererInner {
    type Context = SyncContext<Self>;
}
