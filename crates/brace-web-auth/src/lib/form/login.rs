use brace_web_form::{field, FormBuilder, FormHandler};
use failure::Error;
use uuid::Uuid;

pub struct LoginForm;

impl FormHandler for LoginForm {
    type Context = ();
    type Future = Result<FormBuilder, Error>;

    fn build(&self, mut form: FormBuilder, _: Self::Context) -> Self::Future {
        form.insert(field::hidden("id").value(Uuid::new_v4().to_string()));

        form.insert(
            field::email("email")
                .label("Email")
                .value(form.state().get::<String>("email")?),
        );

        form.insert(field::password("password").label("Password"));

        Ok(form)
    }
}
