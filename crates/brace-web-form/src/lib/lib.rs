use failure::Error;
use futures::future::{loop_fn, ok, Future, FutureResult, IntoFuture, Loop};
use serde::{Deserialize, Serialize};

use self::field::Field;

pub use self::builder::{FormBuilder, FormHandler};
pub use self::state::FormState;

pub mod builder;
pub mod field;
pub mod state;

#[derive(Serialize, Deserialize)]
pub struct Form {
    pub state: FormState,
    pub fields: Vec<Field>,
}

impl Form {
    pub fn build<F>(
        form: F,
        state: FormState,
        ctx: F::Context,
    ) -> impl Future<Item = Form, Error = Error>
    where
        F: FormHandler,
        F::Future: 'static,
    {
        let builder = Box::new(form.build(FormBuilder::new(state), ctx).into_future());

        loop_fn(
            builder as Box<dyn Future<Item = FormBuilder, Error = Error>>,
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

impl IntoFuture for Form {
    type Item = Self;
    type Error = Error;
    type Future = FutureResult<Self::Item, Self::Error>;

    fn into_future(self) -> Self::Future {
        ok(self)
    }
}

impl From<FormBuilder> for Form {
    fn from(form: FormBuilder) -> Self {
        Self {
            state: form.state,
            fields: form.fields,
        }
    }
}
