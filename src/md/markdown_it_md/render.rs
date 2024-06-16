use crate::content::Link;
use crate::content::Page;
use crate::content::SiteAttrs;
use crate::error::Error;
use handlebars::Handlebars;
use serde::Serialize;

use super::TableOfContents;

#[derive(Serialize)]
struct TemplateAttrs<'a> {
    title: String,
    content: String,
    breadcrumb: &'a Vec<Link>,
    toc: Option<&'a TableOfContents>,
    site: &'a SiteAttrs<'a>,
}

impl<'a> TemplateAttrs<'a> {
    fn new(
        title: &str,
        content: &str,
        breadcrumb: &'a Vec<Link>,
        toc: &'a TableOfContents,
        site: &'a SiteAttrs,
    ) -> Self {
        Self {
            title: String::from(title),
            content: String::from(content),
            breadcrumb,
            toc: if !toc.entries.is_empty() {
                Some(toc)
            } else {
                None
            },
            site,
        }
    }
}

#[cfg(feature = "dev-handlebars-templates")]
fn register_templates(h: &mut Handlebars) {
    use std::fs;

    h.register_template_string(
        "page",
        fs::read_to_string("assets/templates/page.hbs").expect("couldn't read page.hbs"),
    )
    .unwrap();

    h.register_template_string(
        "toc",
        fs::read_to_string("assets/templates/toc.hbs").expect("couldn't read toc.hbs"),
    )
    .unwrap();

    h.register_template_string(
        "sidebar",
        fs::read_to_string("assets/templates/sidebar.hbs").expect("couldn't read sidebar.hbs"),
    )
    .unwrap();
}

#[cfg(not(feature = "dev-handlebars-templates"))]
fn register_templates(h: &mut Handlebars) {
    h.register_template_string("page", include_str!("../../../assets/templates/page.hbs"))
        .unwrap();
    h.register_template_string("toc", include_str!("../../../assets/templates/toc.hbs"))
        .unwrap();
    h.register_template_string(
        "sidebar",
        include_str!("../../../assets/templates/sidebar.hbs"),
    )
    .unwrap();
}

#[cfg(feature = "handlebars-templating")]
pub fn render_page(page: &Page, site: &SiteAttrs) -> Result<String, Error> {
    let mut h = Handlebars::new();
    h.set_strict_mode(false);
    register_templates(&mut h);
    let html = page.html.as_str();

    let attrs = TemplateAttrs::new(
        &page.title,
        html,
        &page.breadcrumb,
        &page.table_of_contents,
        site,
    );

    let s = h.render("page", &attrs)?;
    Ok(s)
}
