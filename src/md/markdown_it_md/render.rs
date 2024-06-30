use crate::content::Link;
use crate::content::Page;
use crate::content::SiteAttrs;
use crate::error::JellyError;
use handlebars::Handlebars;
use serde::Serialize;

use super::TableOfContents;

const KEY_PAGE: &str = "page";
const KEY_SIDEBAR: &str = "sidebar";
const KEY_TOC: &str = "toc";

#[derive(Serialize)]
struct TemplateAttrs {
    title: String,
    content: String,
    breadcrumb: Vec<Link>,
    toc: Option<TableOfContents>,
    site: SiteAttrs,
}

impl TemplateAttrs {
    fn new(
        title: &str,
        content: &str,
        breadcrumb: Vec<Link>,
        toc: TableOfContents,
        site: SiteAttrs,
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
fn register_templates(h: &mut Handlebars) -> Result<(), JellyError> {
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
fn register_templates(h: &mut Handlebars) -> Result<(), JellyError> {
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
pub(crate) fn render_page(page: &Page, site: &SiteAttrs) -> Result<String, JellyError> {
    let mut h = Handlebars::new();
    h.set_strict_mode(false);
    register_templates(&mut h)?;
    let html = page.html.as_str();

    let attrs = TemplateAttrs::new(
        &page.title,
        html,
        page.breadcrumb.clone(),
        page.table_of_contents.clone(),
        site.clone(),
    );

    let s = h.render(KEY_PAGE, &attrs)?;
    Ok(s)
}
