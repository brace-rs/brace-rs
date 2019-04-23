use std::collections::VecDeque;

use failure::Error;
use futures::future::{ok, Future, FutureResult, IntoFuture};

use super::field::Field;

type BoxedCallbackWrapper<S> =
    Box<dyn FormCallbackWrapper<S, Future = Box<dyn Future<Item = FormBuilder<S>, Error = Error>>>>;

pub struct FormBuilder<S = ()> {
    pub(crate) state: Box<S>,
    pub(crate) fields: Vec<Field>,
    pub(crate) builders: VecDeque<BoxedCallbackWrapper<S>>,
}

impl<S> FormBuilder<S> {
    pub fn new(state: S) -> Self {
        Self {
            state: Box::new(state),
            fields: Vec::new(),
            builders: VecDeque::new(),
        }
    }
}

impl<S> FormBuilder<S> {
    pub fn state(&self) -> &S {
        &self.state
    }

    pub fn insert<T>(&mut self, field: T) -> &mut Self
    where
        T: Into<Field>,
    {
        self.fields.push(field.into());
        self
    }

    pub fn builder<T>(&mut self, builder: T) -> &mut Self
    where
        T: FormCallbackWrapper<S, Future = Box<dyn Future<Item = FormBuilder<S>, Error = Error>>>
            + 'static,
    {
        self.builders.push_back(Box::new(builder));
        self
    }
}

impl<S> IntoFuture for FormBuilder<S> {
    type Item = Self;
    type Error = Error;
    type Future = FutureResult<Self::Item, Self::Error>;

    fn into_future(self) -> Self::Future {
        ok(self)
    }
}

pub trait FormHandler<S = ()> {
    type Context;
    type Future: IntoFuture<Item = FormBuilder<S>, Error = Error>;

    fn build(&self, form: FormBuilder<S>, ctx: Self::Context) -> Self::Future;
}

pub trait FormCallback<S = ()> {
    type Future: IntoFuture<Item = FormBuilder<S>, Error = Error>;

    fn build(&self, form: FormBuilder<S>) -> Self::Future;
}

impl<S, R, F> FormCallback<S> for F
where
    R: IntoFuture<Item = FormBuilder<S>, Error = Error> + 'static,
    F: Fn(FormBuilder<S>) -> R,
{
    type Future = Box<dyn Future<Item = FormBuilder<S>, Error = Error>>;

    fn build(&self, form: FormBuilder<S>) -> Self::Future {
        Box::new((self)(form).into_future())
    }
}

pub trait FormCallbackWrapper<S = ()> {
    type Future: IntoFuture<Item = FormBuilder<S>, Error = Error>;

    fn build_boxed(&self, form: FormBuilder<S>) -> Self::Future;
}

impl<S, F> FormCallbackWrapper<S> for F
where
    S: 'static,
    F: FormCallback<S> + 'static,
{
    type Future = Box<dyn Future<Item = FormBuilder<S>, Error = Error>>;

    fn build_boxed(&self, form: FormBuilder<S>) -> Self::Future {
        Box::new(self.build(form).into_future())
    }
}
