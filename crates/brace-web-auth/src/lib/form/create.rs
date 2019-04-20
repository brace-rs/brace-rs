use brace_web_form::{field, FormBuilder, FormHandler};
use chrono::Utc;
use failure::Error;
use uuid::Uuid;

pub struct CreateForm;

impl FormHandler for CreateForm {
    type Context = ();

    fn build(&self, form: &mut FormBuilder<()>, _: Self::Context) -> Result<(), Error> {
        form.field(field::hidden("id").value(Uuid::new_v4().to_string()));

        form.field(
            field::text("email")
                .label("Email")
                .description("The email address of the user."),
        );

        form.field(
            field::text("password")
                .label("Password")
                .description("The password of the user."),
        );

        form.field(
            field::datetime("created")
                .label("Created")
                .description("The date/time of when the user was first created.")
                .value(Utc::now()),
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
