use brace_web_form::{action, field, Form, FormBuilder};
use chrono::{DateTime, NaiveDateTime, Utc};
use failure::Error;

pub struct UserForm;

impl FormBuilder for UserForm {
    type Context = ();
    type Future = Result<Form, Error>;

    fn build(&self, mut form: Form, _: Self::Context) -> Self::Future {
        form.insert(field::hidden("id").value(form.state().get::<String>("id")?));

        form.insert(
            field::email("email")
                .label("Email")
                .description("The email address of the user.")
                .value(form.state().get::<String>("email")?),
        );

        form.insert(
            field::password("password")
                .label("Password")
                .description("The password of the user."),
        );

        let created = DateTime::<Utc>::from_utc(
            NaiveDateTime::parse_from_str(
                &form.state().get::<String>("created")?,
                "%Y-%m-%dT%H:%M",
            )?,
            Utc,
        );

        form.insert(
            field::datetime("created")
                .label("Created")
                .description("The date/time of when the user was first created.")
                .value(created),
        );

        form.insert(
            field::datetime("updated")
                .label("Updated")
                .description("The date/time of when the user was last updated.")
                .value(Utc::now()),
        );

        form.action(action::submit(""));
        form.action(action::cancel("/"));

        Ok(form)
    }
}
