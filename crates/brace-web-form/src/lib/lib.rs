use failure::Error;
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
    pub fn build<F>(form: F, state: S, ctx: F::Context) -> Result<Form<S>, Error>
    where
        F: FormHandler<S>,
    {
        let mut builder = FormBuilder::new(state);
        form.build(&mut builder, ctx)?;

        while let Some(callback) = builder.builders.pop_front() {
            (callback)(&mut builder)?;
        }

        Ok(Form {
            state: builder.state,
            fields: builder.fields,
        })
    }
}
