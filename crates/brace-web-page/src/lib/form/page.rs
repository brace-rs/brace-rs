use std::collections::HashMap;

use brace_db::Database;
use brace_web_form::{field, FormBuilder, FormHandler};
use chrono::Utc;
use failure::Error;
use futures::future::Future;

use crate::model::Page;

pub struct PageForm;

impl FormHandler<Page> for PageForm {
    type Context = Database;
    type Future = FormBuilder<Page>;

    fn build(&self, mut form: FormBuilder<Page>, ctx: Self::Context) -> Self::Future {
        form.insert(field::hidden("id").value(form.state().id.to_string()));

        form.insert(
            field::text("title")
                .label("Title")
                .description("The title of the page.")
                .value(form.state().title.clone()),
        );

        form.insert(
            field::text("slug")
                .label("Slug")
                .description("The page slug.")
                .value(form.state().slug.clone()),
        );

        form.insert(
            field::textarea("description")
                .label("Description")
                .description("The description of the page.")
                .value(form.state().description.clone()),
        );

        form.insert(
            field::datetime("created")
                .label("Created")
                .description("The date/time of when the page was first created.")
                .value(form.state().created),
        );

        form.insert(
            field::datetime("updated")
                .label("Updated")
                .description("The date/time of when the page was last updated.")
                .value(Utc::now()),
        );

        form.builder(move |form| build_parent(form, &ctx));

        form
    }
}

fn build_parent(
    mut form: FormBuilder<Page>,
    ctx: &Database,
) -> impl Future<Item = FormBuilder<Page>, Error = Error> {
    crate::action::list::list(&ctx).and_then(|pages| {
        let mut map = HashMap::<String, String>::new();

        for page in pages {
            if form.state().id != page.id {
                map.insert(
                    page.id.to_string(),
                    format!("{} - {}", page.title, page.path),
                );
            }
        }

        form.insert(
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

        form
    })
}
