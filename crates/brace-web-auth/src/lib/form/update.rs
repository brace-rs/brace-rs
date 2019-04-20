use brace_web_form::{field, FormBuilder, FormHandler};
use chrono::Utc;
use failure::Error;

use crate::model::User;

pub struct UpdateForm;

impl FormHandler<User> for UpdateForm {
    type Context = ();

    fn build(&self, form: &mut FormBuilder<User>, _: Self::Context) -> Result<(), Error> {
        form.field(field::hidden("id").value(form.state().id.to_string()));

        form.field(
            field::email("email")
                .label("Email")
                .description("The email address of the user.")
                .value(form.state().email.clone()),
        );

        form.field(
            field::password("password")
                .label("Password")
                .description("The password of the user."),
        );

        form.field(
            field::datetime("created")
                .label("Created")
                .description("The date/time of when the user was first created.")
                .value(form.state().created),
        );

        form.field(
            field::datetime("updated")
                .label("Updated")
                .description("The date/time of when the user was last updated.")
                .value(Utc::now()),
        );

        Ok(())
    }
}
