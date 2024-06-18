use crate::content::Link;
use crate::content::Page;
use crate::content::SiteAttrs;
use crate::error::Error;
use handlebars::Handlebars;
use serde::Serialize;

use super::TableOfContents;

const KEY_PAGE: &str = "page";
const KEY_SIDEBAR: &str = "sidebar";
const KEY_TOC: &str = "toc";

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
fn register_templates(h: &mut Handlebars) -> Result<(), Error> {
    use std::fs;

    h.register_template_string(
        KEY_PAGE,
        fs::read_to_string("assets/templates/handlebars/page.hbs")?,
    )
    .map_err(Box::new)?;
    h.register_template_string(
        KEY_SIDEBAR,
        fs::read_to_string("assets/templates/handlebars/sidebar.hbs")?,
    )
    .map_err(Box::new)?;
    h.register_template_string(
        KEY_TOC,
        fs::read_to_string("assets/templates/handlebars/toc.hbs")?,
    )
    .map_err(Box::new)?;
    Ok(())
}

#[cfg(not(feature = "dev-handlebars-templates"))]
fn register_templates(h: &mut Handlebars) -> Result<(), Error> {
    h.register_template_string(
        KEY_PAGE,
        include_str!("../../../assets/templates/handlebars/page.hbs"),
    )
    .map_err(Box::new)?;
    h.register_template_string(
        KEY_SIDEBAR,
        include_str!("../../../assets/templates/handlebars/sidebar.hbs"),
    )
    .map_err(Box::new)?;
    h.register_template_string(
        KEY_TOC,
        include_str!("../../../assets/templates/handlebars/toc.hbs"),
    )
    .map_err(Box::new)?;
    Ok(())
}

#[cfg(feature = "handlebars-templating")]
pub fn render_page(page: &Page, site: &SiteAttrs) -> Result<String, Error> {
    let mut h = Handlebars::new();
    h.set_strict_mode(false);
    register_templates(&mut h)?;
    let html = page.html.as_str();

    let attrs = TemplateAttrs::new(
        &page.title,
        html,
        &page.breadcrumb,
        &page.table_of_contents,
        site,
    );

    let s = h.render(KEY_PAGE, &attrs)?;
    Ok(s)
}
