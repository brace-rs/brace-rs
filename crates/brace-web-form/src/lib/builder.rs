use failure::Error;
use futures::future::{Future, IntoFuture};

use super::form::Form;

pub trait FormBuilder {
    type Future: IntoFuture<Item = Form, Error = Error>;

    fn build(&self, form: Form) -> Self::Future;
}

impl<R, F> FormBuilder for F
where
    R: IntoFuture<Item = Form, Error = Error> + 'static,
    F: Fn(Form) -> R,
{
    type Future = Box<dyn Future<Item = Form, Error = Error>>;

    fn build(&self, form: Form) -> Self::Future {
        Box::new((self)(form).into_future())
    }
}

pub trait BoxedFormBuilder {
    fn build_boxed(&self, form: Form) -> Box<dyn Future<Item = Form, Error = Error>>;
}

impl<F> BoxedFormBuilder for F
where
    F: FormBuilder + 'static,
{
    fn build_boxed(&self, form: Form) -> Box<dyn Future<Item = Form, Error = Error>> {
        Box::new(self.build(form).into_future())
    }
}
