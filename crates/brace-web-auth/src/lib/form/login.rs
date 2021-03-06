use brace_web_form::{action, field, Form, FormBuilder};
use failure::Error;
use uuid::Uuid;

pub struct LoginForm;

impl FormBuilder for LoginForm {
    type Future = Result<Form, Error>;

    fn build(&self, mut form: Form) -> Self::Future {
        form.insert(field::hidden("id").value(Uuid::new_v4().to_string()));

        form.insert(
            field::email("email")
                .label("Email")
                .value(form.data().get::<String>("email")?),
        );

        form.insert(field::password("password").label("Password"));

        form.action(action::submit(""));
        form.action(action::cancel("/"));

        Ok(form)
    }
}
