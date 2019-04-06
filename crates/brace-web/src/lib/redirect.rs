use actix_web::http::header::LOCATION;
use actix_web::http::StatusCode;
use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ok, FutureResult, IntoFuture};

pub struct HttpRedirect(StatusCode, String);

impl HttpRedirect {
    pub fn to<U: Into<String>>(uri: U) -> Self {
        Self(StatusCode::SEE_OTHER, uri.into())
    }

    pub fn temporary<U: Into<String>>(uri: U) -> Self {
        Self(StatusCode::TEMPORARY_REDIRECT, uri.into())
    }

    pub fn permanent<U: Into<String>>(uri: U) -> Self {
        Self(StatusCode::PERMANENT_REDIRECT, uri.into())
    }

    pub fn found<U: Into<String>>(uri: U) -> Self {
        Self(StatusCode::FOUND, uri.into())
    }

    pub fn moved<U: Into<String>>(uri: U) -> Self {
        Self(StatusCode::MOVED_PERMANENTLY, uri.into())
    }

    pub fn into_response(self) -> HttpResponse {
        HttpResponse::build(self.0)
            .header(LOCATION, self.1)
            .finish()
    }
}

impl Responder for HttpRedirect {
    type Error = Error;
    type Future = FutureResult<HttpResponse, Error>;

    fn respond_to(self, _: &HttpRequest) -> Self::Future {
        ok(self.into_response())
    }
}

impl IntoFuture for HttpRedirect {
    type Item = HttpRedirect;
    type Error = Error;
    type Future = FutureResult<HttpRedirect, Error>;

    fn into_future(self) -> Self::Future {
        ok(self)
    }
}

impl Into<HttpResponse> for HttpRedirect {
    fn into(self) -> HttpResponse {
        self.into_response()
    }
}
