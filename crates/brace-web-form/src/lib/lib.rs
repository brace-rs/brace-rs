use failure::Error;
use futures::future::{loop_fn, ok, Future, FutureResult, IntoFuture, Loop};
use serde::{Deserialize, Serialize};

use self::field::Field;

pub use self::builder::{FormBuilder, FormHandler};

pub mod builder;
pub mod field;

#[derive(Serialize, Deserialize)]
pub struct Form<S = ()> {
    pub state: Box<S>,
    pub fields: Vec<Field>,
}

impl<S> Form<S> {
    pub fn build<F>(
        form: F,
        state: S,
        ctx: F::Context,
    ) -> impl Future<Item = Form<S>, Error = Error>
    where
        F: FormHandler<S>,
        F::Future: 'static,
    {
        let builder = Box::new(form.build(FormBuilder::new(state), ctx).into_future());

        loop_fn(
            builder as Box<dyn Future<Item = FormBuilder<S>, Error = Error>>,
            |form| {
                form.into_future()
                    .and_then(|mut form| match form.builders.pop_front() {
                        Some(next) => Ok(Loop::Continue(next.build_boxed(form))),
                        None => Ok(Loop::Break(form)),
                    })
            },
        )
        .map(Form::from)
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

impl<S> From<FormBuilder<S>> for Form<S> {
    fn from(form: FormBuilder<S>) -> Self {
        Self {
            state: form.state,
            fields: form.fields,
        }
    }
}
