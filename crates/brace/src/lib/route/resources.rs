use std::cell::RefCell;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;

use actix_files::NamedFile;
use actix_service::boxed::{self, BoxedNewService, BoxedService};
use actix_service::{IntoNewService, NewService, Service};
use actix_web::dev::{
    HttpServiceFactory, Payload, ResourceDef, ServiceConfig, ServiceRequest, ServiceResponse,
};
use actix_web::error::Error;
use actix_web::{HttpRequest, Responder};
use brace_config::load;
use brace_theme::config::ThemeConfig;
use brace_theme::manifest::ManifestConfig;
use brace_theme::resource::ResourceInfo;
use futures::future::{ok, Either, Future, FutureResult};
use futures::{Async, Poll};
use serde::Deserialize;

type HttpService<P> = BoxedService<ServiceRequest<P>, ServiceResponse, Error>;
type HttpNewService<P> = BoxedNewService<(), ServiceRequest<P>, ServiceResponse, Error, ()>;
type FutureResponse = Box<Future<Item = ServiceResponse, Error = Error>>;

#[derive(Deserialize)]
pub struct ThemeResource {
    pub theme: String,
    pub kind: String,
    pub resource: String,
}

pub struct ThemeResources<S> {
    path: String,
    default: Rc<RefCell<Option<Rc<HttpNewService<S>>>>>,
    themes: Vec<(ThemeConfig, PathBuf)>,
}

impl<S: 'static> ThemeResources<S> {
    pub fn new(path: &str, themes: Vec<(ThemeConfig, PathBuf)>) -> Self {
        Self {
            path: path.to_string(),
            default: Rc::new(RefCell::new(None)),
            themes,
        }
    }

    pub fn default_handler<F, U>(mut self, f: F) -> Self
    where
        F: IntoNewService<U>,
        U: NewService<Request = ServiceRequest<S>, Response = ServiceResponse, Error = Error>
            + 'static,
    {
        self.default = Rc::new(RefCell::new(Some(Rc::new(boxed::new_service(
            f.into_new_service().map_init_err(|_| ()),
        )))));

        self
    }
}

impl<S: 'static> HttpServiceFactory<S> for ThemeResources<S> {
    fn register(self, config: &mut ServiceConfig<S>) {
        if self.default.borrow().is_none() {
            *self.default.borrow_mut() = Some(config.default_service());
        }

        let rdef = if config.is_root() {
            ResourceDef::root_prefix(&self.path)
        } else {
            ResourceDef::prefix(&self.path)
        };

        config.register_service(rdef, None, self, None)
    }
}

impl<S: 'static> NewService for ThemeResources<S> {
    type Request = ServiceRequest<S>;
    type Response = ServiceResponse;
    type Error = Error;
    type Service = ThemeResourcesService<S>;
    type InitError = ();
    type Future = Box<Future<Item = Self::Service, Error = Self::InitError>>;

    fn new_service(&self, _: &()) -> Self::Future {
        let mut srv = ThemeResourcesService {
            default: None,
            themes: self.themes.clone(),
        };

        if let Some(ref default) = *self.default.borrow() {
            Box::new(
                default
                    .new_service(&())
                    .map(move |default| {
                        srv.default = Some(default);
                        srv
                    })
                    .map_err(|_| ()),
            )
        } else {
            Box::new(ok(srv))
        }
    }
}

pub struct ThemeResourcesService<P> {
    default: Option<HttpService<P>>,
    themes: Vec<(ThemeConfig, PathBuf)>,
}

impl<P> ThemeResourcesService<P> {
    fn handle_err(
        &mut self,
        err: std::io::Error,
        req: HttpRequest,
        payload: Payload<P>,
    ) -> Either<FutureResult<ServiceResponse, Error>, FutureResponse> {
        log::debug!("ThemeResources: Failed to handle {}: {}", req.path(), err);
        if let Some(ref mut default) = self.default {
            default.call(ServiceRequest::from_parts(req, payload))
        } else {
            Either::A(ok(ServiceResponse::from_err(err, req.clone())))
        }
    }
}

impl<P> Service for ThemeResourcesService<P> {
    type Request = ServiceRequest<P>;
    type Response = ServiceResponse;
    type Error = Error;
    type Future = Either<FutureResult<Self::Response, Self::Error>, FutureResponse>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        Ok(Async::Ready(()))
    }

    fn call(&mut self, req: ServiceRequest<P>) -> Self::Future {
        let (req, payload) = req.into_parts();

        let mut path = req.match_info().clone();
        let rdef = ResourceDef::new("/{theme}/{kind}/{resource:.*}");

        if rdef.match_path(&mut path) {
            if let Ok(ThemeResource {
                theme,
                kind,
                resource,
            }) = path.load()
            {
                let res = find_theme(&theme, &self.themes).and_then(|(theme, theme_path)| {
                    find_resource(&resource, theme, &theme_path).and_then(|mut resource| {
                        match resource {
                            ResourceInfo::StyleSheet(ref mut info) => {
                                if &kind == "css" {
                                    if info.location.is_internal() {
                                        info.location = theme_path
                                            .join(info.location.clone().into_inner())
                                            .into();

                                        Some(resource)
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            ResourceInfo::JavaScript(ref mut info) => {
                                if &kind == "js" {
                                    if info.location.is_internal() {
                                        info.location = theme_path
                                            .join(info.location.clone().into_inner())
                                            .into();

                                        Some(resource)
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                        }
                    })
                });

                if let Some(res) = res {
                    return match NamedFile::open(res.location().to_string()) {
                        Ok(named_file) => match named_file.respond_to(&req) {
                            Ok(item) => Either::A(ok(ServiceResponse::new(req.clone(), item))),
                            Err(err) => Either::A(ok(ServiceResponse::from_err(err, req.clone()))),
                        },
                        Err(err) => self.handle_err(err, req, payload),
                    };
                }
            }
        }

        self.handle_err(
            IoError::new(IoErrorKind::NotFound, "Resource not found"),
            req,
            payload,
        )
    }
}

fn load_manifests(theme: ThemeConfig, path: &Path) -> Vec<ManifestConfig> {
    theme
        .manifests
        .iter()
        .filter_map(|manifest| match load::file(path.join(&manifest.path)) {
            Ok(conf) => Some(conf),
            Err(_) => None,
        })
        .collect()
}

fn find_theme(name: &str, themes: &[(ThemeConfig, PathBuf)]) -> Option<(ThemeConfig, PathBuf)> {
    themes.iter().find_map(|(theme, path)| {
        if theme.theme.name == name {
            if let Some(path) = path.parent() {
                Some((theme.clone(), path.to_path_buf()))
            } else {
                None
            }
        } else {
            None
        }
    })
}

fn find_resource(name: &str, theme: ThemeConfig, path: &Path) -> Option<ResourceInfo> {
    load_manifests(theme, path)
        .iter()
        .map(|manifest| manifest.resources.clone())
        .flatten()
        .find_map(|resource| {
            if resource.name() == name {
                Some(resource.clone())
            } else {
                None
            }
        })
}
