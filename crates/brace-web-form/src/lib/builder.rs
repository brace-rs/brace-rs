use failure::Error;
use futures::future::{Future, IntoFuture};

use super::form::Form;

pub trait FormBuilder {
    type Future: IntoFuture<Item = Form, Error = Error>;

    fn build(&self, form: Form) -> Self::Future;
}

pub trait FormCallback {
    type Future: IntoFuture<Item = Form, Error = Error>;

    fn build(&self, form: Form) -> Self::Future;
}

impl<R, F> FormCallback for F
where
    R: IntoFuture<Item = Form, Error = Error> + 'static,
    F: Fn(Form) -> R,
{
    type Future = Box<dyn Future<Item = Form, Error = Error>>;

    fn build(&self, form: Form) -> Self::Future {
        Box::new((self)(form).into_future())
    }
}

pub trait FormCallbackWrapper {
    type Future: IntoFuture<Item = Form, Error = Error>;

    fn build_boxed(&self, form: Form) -> Self::Future;
}

impl<F> FormCallbackWrapper for F
where
    F: FormCallback + 'static,
{
    type Future = Box<dyn Future<Item = Form, Error = Error>>;

    fn build_boxed(&self, form: Form) -> Self::Future {
        Box::new(self.build(form).into_future())
    }
}
