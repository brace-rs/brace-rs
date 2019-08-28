use std::collections::VecDeque;

use failure::Error;
use futures::future::{loop_fn, ok, Future, FutureResult, IntoFuture, Loop};
use serde::{Deserialize, Serialize};

use super::action::Action;
use super::builder::BoxedFormBuilder;
use super::data::FormData;
use super::field::Field;

#[derive(Serialize, Deserialize)]
pub struct Form<S = ()> {
    pub(crate) data: FormData,
    pub(crate) state: Box<S>,
    pub(crate) fields: Vec<Field>,
    pub(crate) actions: Vec<Action>,
    #[serde(skip, default = "VecDeque::new")]
    pub(crate) builders: VecDeque<Box<dyn BoxedFormBuilder<S>>>,
}

impl<S> Form<S>
where
    S: 'static,
{
    pub fn new(state: S) -> Self {
        Self {
            data: FormData::new(),
            state: Box::new(state),
            fields: Vec::new(),
            actions: Vec::new(),
            builders: VecDeque::new(),
        }
    }

    pub fn with(mut self, data: FormData) -> Self {
        self.data = data;
        self
    }

    pub fn build(self) -> impl Future<Item = Self, Error = Error> {
        let form = Box::new(self.into_future());

        loop_fn(
            form as Box<dyn Future<Item = Form<S>, Error = Error>>,
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

#[cfg(test)]
mod tests {
    use futures::future::Future;

    use crate::{action, field, Form};

    struct FormState {
        value: String,
    }

    fn build_form_without_state(mut form: Form) -> Form {
        form.insert(field::hidden("field").value("value".to_owned()));
        form.action(action::submit("submit"));
        form
    }

    fn build_form_with_state(mut form: Form<FormState>) -> Form<FormState> {
        form.insert(field::hidden("field").value(form.state().value.clone()));
        form.action(action::submit("submit"));
        form
    }

    #[test]
    fn test_form_build_without_state() {
        let mut form = Form::new(());

        form.builder(build_form_without_state);

        let form = form.build().wait().unwrap();

        assert_eq!(form.fields.len(), 1);
        assert_eq!(form.actions.len(), 1);
    }

    #[test]
    fn test_form_build_with_state() {
        let state = FormState {
            value: "Hello".to_owned(),
        };
        let mut form = Form::new(state);

        form.builder(build_form_with_state);

        let form = form.build().wait().unwrap();

        assert_eq!(form.fields.len(), 1);
        assert_eq!(form.actions.len(), 1);
    }
}
