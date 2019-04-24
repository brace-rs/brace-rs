use std::collections::VecDeque;

use failure::Error;
use futures::future::{ok, Future, FutureResult, IntoFuture};

use super::field::Field;
use super::state::FormState;

type BoxedCallbackWrapper =
    Box<dyn FormCallbackWrapper<Future = Box<dyn Future<Item = FormBuilder, Error = Error>>>>;

pub struct FormBuilder {
    pub(crate) state: FormState,
    pub(crate) fields: Vec<Field>,
    pub(crate) builders: VecDeque<BoxedCallbackWrapper>,
}

impl FormBuilder {
    pub fn new(state: FormState) -> Self {
        Self {
            state,
            fields: Vec::new(),
            builders: VecDeque::new(),
        }
    }
}

impl FormBuilder {
    pub fn state(&self) -> &FormState {
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
        T: FormCallbackWrapper<Future = Box<dyn Future<Item = FormBuilder, Error = Error>>>
            + 'static,
    {
        self.builders.push_back(Box::new(builder));
        self
    }
}

impl IntoFuture for FormBuilder {
    type Item = Self;
    type Error = Error;
    type Future = FutureResult<Self::Item, Self::Error>;

    fn into_future(self) -> Self::Future {
        ok(self)
    }
}

pub trait FormHandler {
    type Context;
    type Future: IntoFuture<Item = FormBuilder, Error = Error>;

    fn build(&self, form: FormBuilder, ctx: Self::Context) -> Self::Future;
}

pub trait FormCallback {
    type Future: IntoFuture<Item = FormBuilder, Error = Error>;

    fn build(&self, form: FormBuilder) -> Self::Future;
}

impl<R, F> FormCallback for F
where
    R: IntoFuture<Item = FormBuilder, Error = Error> + 'static,
    F: Fn(FormBuilder) -> R,
{
    type Future = Box<dyn Future<Item = FormBuilder, Error = Error>>;

    fn build(&self, form: FormBuilder) -> Self::Future {
        Box::new((self)(form).into_future())
    }
}

pub trait FormCallbackWrapper {
    type Future: IntoFuture<Item = FormBuilder, Error = Error>;

    fn build_boxed(&self, form: FormBuilder) -> Self::Future;
}

impl<F> FormCallbackWrapper for F
where
    F: FormCallback + 'static,
{
    type Future = Box<dyn Future<Item = FormBuilder, Error = Error>>;

    fn build_boxed(&self, form: FormBuilder) -> Self::Future {
        Box::new(self.build(form).into_future())
    }
}
