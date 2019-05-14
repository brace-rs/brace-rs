use std::collections::HashMap;

use brace_db::Database;
use brace_web_form::{action, field, Form, FormBuilder};
use chrono::{DateTime, NaiveDateTime, Utc};
use failure::Error;
use futures::future::Future;
use uuid::Uuid;

pub struct PageForm {
    pub database: Database,
}

impl FormBuilder for PageForm {
    type Future = Result<Form, Error>;

    fn build(&self, mut form: Form) -> Self::Future {
        form.insert(field::hidden("id").value(form.data().get::<String>("id")?));

        form.insert(
            field::text("title")
                .label("Title")
                .description("The title of the page.")
                .value(form.data().get::<String>("title")?),
        );

        form.insert(
            field::text("slug")
                .label("Slug")
                .description("The page slug.")
                .value(form.data().get::<String>("slug")?),
        );

        form.insert(
            field::textarea("description")
                .label("Description")
                .description("The description of the page.")
                .value(form.data().get::<String>("description")?),
        );

        let created = DateTime::<Utc>::from_utc(
            NaiveDateTime::parse_from_str(
                &form.data().get::<String>("created")?,
                "%Y-%m-%dT%H:%M",
            )?,
            Utc,
        );

        form.insert(
            field::datetime("created")
                .label("Created")
                .description("The date/time of when the page was first created.")
                .value(created),
        );

        form.insert(
            field::datetime("updated")
                .label("Updated")
                .description("The date/time of when the page was last updated.")
                .value(Utc::now()),
        );

        form.action(action::submit(""));
        form.action(action::cancel("/"));

        let db = self.database.clone();
        form.builder(move |form| build_parent(form, db.clone()));

        Ok(form)
    }
}

fn build_parent(mut form: Form, ctx: Database) -> impl Future<Item = Form, Error = Error> {
    crate::action::list::list(&ctx).and_then(|pages| {
        let mut map = HashMap::<String, String>::new();

        for page in pages {
            if form.data().get::<Uuid>("id")? != page.id {
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
                    form.data()
                        .get::<String>("parent")
                        .unwrap_or_else(|_| "".to_owned()),
                )
                .options(map),
        );

        Ok(form)
    })
}
