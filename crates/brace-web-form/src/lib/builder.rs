use failure::Error;

use super::field::Field;

pub struct FormBuilder<S = ()> {
    pub(crate) state: Box<S>,
    pub(crate) fields: Vec<Field>,
}

impl<S> FormBuilder<S> {
    pub fn new(state: S) -> Self {
        Self {
            state: Box::new(state),
            fields: Vec::new(),
        }
    }
}

impl<S> FormBuilder<S> {
    pub fn state(&self) -> &S {
        &self.state
    }

    pub fn field<T>(&mut self, field: T) -> &mut Self
    where
        T: Into<Field>,
    {
        self.fields.push(field.into());
        self
    }
}

pub trait FormHandler<S = ()> {
    type Context: Default;

    fn build(&self, _form: &mut FormBuilder<S>, _ctx: Self::Context) -> Result<(), Error> {
        Ok(())
    }
}
