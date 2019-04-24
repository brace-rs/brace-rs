use std::collections::VecDeque;

use failure::Error;
use futures::future::{loop_fn, ok, Future, FutureResult, IntoFuture, Loop};
use serde::{Deserialize, Serialize};

use super::builder::{FormBuilder, FormCallbackWrapper};
use super::field::Field;
use super::state::FormState;

type BoxedCallbackWrapper =
    Box<dyn FormCallbackWrapper<Future = Box<dyn Future<Item = Form, Error = Error>>>>;

#[derive(Serialize, Deserialize)]
pub struct Form {
    pub(crate) state: FormState,
    pub(crate) fields: Vec<Field>,
    #[serde(skip, default = "VecDeque::new")]
    pub(crate) builders: VecDeque<BoxedCallbackWrapper>,
}

impl Form {
    pub fn new(state: FormState) -> Self {
        Self {
            state,
            fields: Vec::new(),
            builders: VecDeque::new(),
        }
    }

    pub fn build<F>(
        form: F,
        state: FormState,
        ctx: F::Context,
    ) -> impl Future<Item = Self, Error = Error>
    where
        F: FormBuilder,
        F::Future: 'static,
    {
        let builder = Box::new(form.build(Form::new(state), ctx).into_future());

        loop_fn(
            builder as Box<dyn Future<Item = Form, Error = Error>>,
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

impl Form {
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
        T: FormCallbackWrapper<Future = Box<dyn Future<Item = Form, Error = Error>>> + 'static,
    {
        self.builders.push_back(Box::new(builder));
        self
    }
}

impl IntoFuture for Form {
    type Item = Self;
    type Error = Error;
    type Future = FutureResult<Self::Item, Self::Error>;

    fn into_future(self) -> Self::Future {
        ok(self)
    }
}
