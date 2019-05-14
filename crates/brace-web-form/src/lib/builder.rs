use failure::Error;
use futures::future::{Future, IntoFuture};

use super::form::Form;

pub trait FormBuilder<S = ()> {
    type Future: IntoFuture<Item = Form<S>, Error = Error>;

    fn build(&self, form: Form<S>) -> Self::Future;
}

impl<S, R, F> FormBuilder<S> for F
where
    R: IntoFuture<Item = Form<S>, Error = Error> + 'static,
    F: Fn(Form<S>) -> R,
{
    type Future = Box<dyn Future<Item = Form<S>, Error = Error>>;

    fn build(&self, form: Form<S>) -> Self::Future {
        Box::new((self)(form).into_future())
    }
}

pub trait BoxedFormBuilder<S> {
    fn build_boxed(&self, form: Form<S>) -> Box<dyn Future<Item = Form<S>, Error = Error>>;
}

impl<S, F> BoxedFormBuilder<S> for F
where
    S: 'static,
    F: FormBuilder<S> + 'static,
{
    fn build_boxed(&self, form: Form<S>) -> Box<dyn Future<Item = Form<S>, Error = Error>> {
        Box::new(self.build(form).into_future())
    }
}
