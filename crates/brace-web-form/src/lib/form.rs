use std::collections::VecDeque;

use failure::Error;
use futures::future::{loop_fn, ok, Future, FutureResult, IntoFuture, Loop};
use serde::{Deserialize, Serialize};

use super::action::Action;
use super::builder::{BoxedFormBuilder, FormBuilder};
use super::data::FormData;
use super::field::Field;

#[derive(Serialize, Deserialize)]
pub struct Form<S = ()> {
    pub(crate) data: FormData,
    pub(crate) state: Box<S>,
    pub(crate) fields: Vec<Field>,
    pub(crate) actions: Vec<Action>,
    #[serde(skip, default = "VecDeque::new")]
    pub(crate) builders: VecDeque<Box<BoxedFormBuilder<S>>>,
}

impl<S> Form<S> {
    pub fn new(state: S, data: FormData) -> Self {
        Self {
            data,
            state: Box::new(state),
            fields: Vec::new(),
            actions: Vec::new(),
            builders: VecDeque::new(),
        }
    }

    pub fn build<F>(form: F, state: S, data: FormData) -> impl Future<Item = Self, Error = Error>
    where
        F: FormBuilder<S>,
        F::Future: 'static,
    {
        let builder = Box::new(form.build(Form::new(state, data)).into_future());

        loop_fn(
            builder as Box<dyn Future<Item = Form<S>, Error = Error>>,
            |form| {
                form.into_future()
                    .and_then(|mut form| match form.builders.pop_front() {
                        Some(next) => Ok(Loop::Continue(next.build_boxed(form))),
                        None => Ok(Loop::Break(form)),
                    })
            },
        )
    }
}

impl<S> Form<S> {
    pub fn data(&self) -> &FormData {
        &self.data
    }

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

    pub fn action<T>(&mut self, action: T) -> &mut Self
    where
        T: Into<Action>,
    {
        self.actions.push(action.into());
        self
    }

    pub fn builder<T>(&mut self, builder: T) -> &mut Self
    where
        T: BoxedFormBuilder<S> + 'static,
    {
        self.builders.push_back(Box::new(builder));
        self
    }
}

impl<S> IntoFuture for Form<S> {
    type Item = Self;
    type Error = Error;
    type Future = FutureResult<Self::Item, Self::Error>;

    fn into_future(self) -> Self::Future {
        ok(self)
    }
}
