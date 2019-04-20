use std::collections::HashMap;

use brace_web_form::{field, FormBuilder, FormHandler};
use chrono::Utc;
use failure::Error;
use uuid::Uuid;

use crate::model::PageWithPath;

pub struct CreateForm;

impl FormHandler for CreateForm {
    type Context = Vec<PageWithPath>;

    fn build(&self, form: &mut FormBuilder<()>, ctx: Self::Context) -> Result<(), Error> {
        let mut map = HashMap::<String, String>::new();

        for page in ctx {
            map.insert(
                page.id.to_string(),
                format!("{} - {}", page.title, page.path),
            );
        }

        form.field(field::hidden("id").value(Uuid::new_v4().to_string()));

        form.field(
            field::text("title")
                .label("Title")
                .description("The title of the page."),
        );

        form.field(
            field::text("slug")
                .label("Slug")
                .description("The page slug."),
        );

        form.field(
            field::textarea("description")
                .label("Description")
                .description("The description of the page."),
        );

        form.field(
            field::select("parent")
                .label("Parent")
                .description("The parent page")
                .options(map),
        );

        form.field(
            field::datetime("created")
                .label("Created")
                .description("The date/time of when the page was first created.")
                .value(Utc::now()),
        );

        form.field(
            field::datetime("updated")
                .label("Updated")
                .description("The date/time of when the page was last updated.")
                .value(Utc::now()),
        );

        Ok(())
    }
}
