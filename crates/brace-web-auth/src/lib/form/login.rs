use brace_web_form::{field, FormBuilder, FormHandler};
use failure::Error;
use uuid::Uuid;

use crate::model::UserAuth;

pub struct LoginForm;

impl FormHandler<UserAuth> for LoginForm {
    type Context = ();

    fn build(&self, form: &mut FormBuilder<UserAuth>, _: Self::Context) -> Result<(), Error> {
        form.field(field::hidden("id").value(Uuid::new_v4().to_string()));

        form.field(
            field::email("email")
                .label("Email")
                .value(form.state().email.clone()),
        );

        form.field(field::password("password").label("Password"));

        Ok(())
    }
}
