use crate::config::SiteConfig;
use crate::error::Error;
use handlebars::Handlebars;
use page::Page;
use serde_json::json;
use std::{
    fs::{metadata, read_dir},
    path::PathBuf,
};

pub mod cmd;
pub mod config;
pub mod content;
pub mod error;
pub mod md;
pub mod page;
pub mod site;
pub mod title;
pub mod utils;

#[allow(dead_code)]
fn render_page(page: &Page) -> Result<String, Error> {
    let mut h = Handlebars::new();
    h.set_strict_mode(true);
    let template = include_str!("template/page.hbs");
    let _ = h.register_template_string("html", template);
    let html = page.html.as_str();
    let s = h.render(
        "html",
        &json!({ "content": html, "title": page.title, "breadcrumb": page.breadcrumb }),
    )?;
    Ok(s)
}

pub fn get_pages_in_dir(
    dir: &PathBuf,
    breadcrumb: &[(&PathBuf, &str)],
    config: &SiteConfig,
) -> Result<Vec<Page>, Error> {
    let mut pages: Vec<Page> = Vec::new();

    for entry in read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let meta = metadata(&path)?;
        if meta.is_file() {
            let ext = path.extension();

            if ext.is_some() && ext.unwrap().to_string_lossy().ends_with("md") {
                let page = Page::from_path(&path, breadcrumb, config)?;
                pages.push(page);
            }
        }
    }

    if pages.is_empty() {
        return Err(Error::NoPages(String::from(dir.to_string_lossy())));
    }

    Ok(pages)
}
