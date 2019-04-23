use brace_web_form::{field, FormBuilder, FormHandler};
use uuid::Uuid;

use crate::model::UserAuth;

pub struct LoginForm;

impl FormHandler<UserAuth> for LoginForm {
    type Context = ();
    type Future = FormBuilder<UserAuth>;

    fn build(&self, mut form: FormBuilder<UserAuth>, _: Self::Context) -> Self::Future {
        form.insert(field::hidden("id").value(Uuid::new_v4().to_string()));

        form.insert(
            field::email("email")
                .label("Email")
                .value(form.state().email.clone()),
        );

        form.insert(field::password("password").label("Password"));

        form
    }
}
