use std::collections::VecDeque;

use failure::Error;

use super::field::Field;

pub struct FormBuilder<S = ()> {
    pub(crate) state: Box<S>,
    pub(crate) fields: Vec<Field>,
    pub(crate) builders: VecDeque<Box<Fn(&mut FormBuilder<S>) -> Result<(), Error>>>,
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
        T: 'static + Fn(&mut FormBuilder<S>) -> Result<(), Error>,
    {
        self.builders.push_back(Box::new(builder));
        self
    }
}

pub trait FormHandler<S = ()> {
    type Context;

    fn build(&self, form: &mut FormBuilder<S>, ctx: Self::Context) -> Result<(), Error>;
}
