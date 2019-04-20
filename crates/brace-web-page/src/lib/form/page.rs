use std::collections::HashMap;

use brace_web_form::{field, FormBuilder, FormHandler};
use chrono::Utc;
use failure::Error;

use crate::model::{Page, PageWithPath};

pub struct PageForm;

impl FormHandler<Page> for PageForm {
    type Context = Vec<PageWithPath>;

    fn build(&self, form: &mut FormBuilder<Page>, ctx: Self::Context) -> Result<(), Error> {
        let mut map = HashMap::<String, String>::new();

        for page in ctx {
            if form.state().id != page.id {
                map.insert(
                    page.id.to_string(),
                    format!("{} - {}", page.title, page.path),
                );
            }
        }

        form.field(field::hidden("id").value(form.state().id.to_string()));

        form.field(
            field::text("title")
                .label("Title")
                .description("The title of the page.")
                .value(form.state().title.clone()),
        );

        form.field(
            field::text("slug")
                .label("Slug")
                .description("The page slug.")
                .value(form.state().slug.clone()),
        );

        form.field(
            field::textarea("description")
                .label("Description")
                .description("The description of the page.")
                .value(form.state().description.clone()),
        );

        form.field(
            field::select("parent")
                .label("Parent")
                .description("The parent page.")
                .value(
                    form.state()
                        .parent
                        .map(|id| id.to_string())
                        .unwrap_or_else(|| "".to_owned()),
                )
                .options(map),
        );

        form.field(
            field::datetime("created")
                .label("Created")
                .description("The date/time of when the page was first created.")
                .value(form.state().created),
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
