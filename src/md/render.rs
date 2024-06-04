use crate::content::Link;
use crate::content::Page;
use crate::error::Error;
use handlebars::Handlebars;
use serde::Serialize;

use super::TableOfContents;

#[derive(Serialize)]
struct TemplateAttrs<'a> {
    title: String,
    content: String,
    breadcrumb: &'a Vec<Link>,
    toc: &'a TableOfContents,
}

impl<'a> TemplateAttrs<'a> {
    fn new(
        title: &str,
        content: &str,
        breadcrumb: &'a Vec<Link>,
        toc: &'a TableOfContents,
    ) -> Self {
        Self {
            title: String::from(title),
            content: String::from(content),
            breadcrumb,
            toc,
        }
    }
}

#[cfg(feature = "handlebars-templating")]
pub fn render_page(page: &Page) -> Result<String, Error> {
    let mut h = Handlebars::new();
    h.set_strict_mode(true);
    let template = include_str!("../template/page.hbs");
    let _ = h.register_template_string("html", template);
    let html = page.html.as_str();

    let attrs = TemplateAttrs::new(&page.title, html, &page.breadcrumb, &page.table_of_contents);

    let s = h.render("html", &attrs)?;
    Ok(s)
}
