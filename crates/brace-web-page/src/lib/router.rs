use std::cell::RefCell;
use std::rc::Rc;

use actix_service::boxed::{self, BoxedNewService, BoxedService};
use actix_service::{IntoNewService, NewService, Service};
use actix_web::dev::{
    AppService, HttpServiceFactory, ResourceDef, ServiceRequest, ServiceResponse,
};
use actix_web::error::{Error, ErrorNotFound};
use brace_db::Database;
use brace_web::render::Renderer;
use futures::future::{ok, Either, Future, FutureResult};
use futures::{Async, Poll};

type HttpService = BoxedService<ServiceRequest, ServiceResponse, Error>;
type HttpNewService = BoxedNewService<(), ServiceRequest, ServiceResponse, Error, ()>;
type FutureResponse = Box<Future<Item = ServiceResponse, Error = Error>>;

pub struct PageRouter {
    path: String,
    default: Rc<RefCell<Option<Rc<HttpNewService>>>>,
}

impl PageRouter {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
            default: Rc::new(RefCell::new(None)),
        }
    }

    pub fn default_handler<F, U>(mut self, f: F) -> Self
    where
        F: IntoNewService<U>,
        U: NewService<
                Config = (),
                Request = ServiceRequest,
                Response = ServiceResponse,
                Error = Error,
            > + 'static,
    {
        self.default = Rc::new(RefCell::new(Some(Rc::new(boxed::new_service(
            f.into_new_service().map_init_err(|_| ()),
        )))));

        self
    }
}

impl HttpServiceFactory for PageRouter {
    fn register(self, config: &mut AppService) {
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

impl NewService for PageRouter {
    type Request = ServiceRequest;
    type Response = ServiceResponse;
    type Error = Error;
    type Service = PageRouterService;
    type InitError = ();
    type Future = Box<Future<Item = Self::Service, Error = Self::InitError>>;
    type Config = ();

    fn new_service(&self, _: &()) -> Self::Future {
        let mut srv = PageRouterService { default: None };

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

pub struct PageRouterService {
    default: Option<HttpService>,
}

impl PageRouterService {
    fn handle_err(
        &mut self,
        err: Error,
        req: ServiceRequest,
    ) -> Either<FutureResult<ServiceResponse, Error>, FutureResponse> {
        if let Some(ref mut default) = self.default {
            default.call(req)
        } else {
            Either::A(ok(req.error_response(err)))
        }
    }
}

impl Service for PageRouterService {
    type Request = ServiceRequest;
    type Response = ServiceResponse;
    type Error = Error;
    type Future = Either<FutureResult<Self::Response, Self::Error>, FutureResponse>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        Ok(Async::Ready(()))
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let database = req.app_data::<Database>();
        let renderer = req.app_data::<Renderer>();

        if let Some(database) = database {
            if let Some(renderer) = renderer {
                let (req, _) = req.into_parts();

                return Either::B(Box::new(
                    crate::route::web::locate::get(req.clone(), database, renderer)
                        .map_err(ErrorNotFound)
                        .then(move |res| match res {
                            Ok(res) => ServiceResponse::new(req, res),
                            Err(err) => ServiceResponse::from_err(err, req),
                        }),
                ));
            }
        }

        self.handle_err(ErrorNotFound("Page not found"), req)
    }
}
