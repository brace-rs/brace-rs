use brace_web_form::{field, FormBuilder, FormHandler};
use chrono::Utc;
use failure::Error;

use crate::model::User;

pub struct UserForm;

impl FormHandler<User> for UserForm {
    type Context = ();

    fn build(&self, form: &mut FormBuilder<User>, _: Self::Context) -> Result<(), Error> {
        form.insert(field::hidden("id").value(form.state().id.to_string()));

        form.insert(
            field::email("email")
                .label("Email")
                .description("The email address of the user.")
                .value(form.state().email.clone()),
        );

        form.insert(
            field::password("password")
                .label("Password")
                .description("The password of the user."),
        );

        form.insert(
            field::datetime("created")
                .label("Created")
                .description("The date/time of when the user was first created.")
                .value(form.state().created),
        );

        form.insert(
            field::datetime("updated")
                .label("Updated")
                .description("The date/time of when the user was last updated.")
                .value(Utc::now()),
        );

        Ok(())
    }
}
