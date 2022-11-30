use crate::config::Config;
use crate::error::ContentError;
use handlebars::Handlebars;
use page::Page;
use serde_json::json;
use std::{
    fs::{metadata, read_dir},
    path::Path,
};

pub mod config;
pub mod error;
pub mod page;
pub mod section;
pub mod site;
pub mod utils;

#[allow(dead_code)]
fn render_page(page: &Page) -> Result<String, ContentError> {
    let mut h = Handlebars::new();
    h.set_strict_mode(true);
    let template = include_str!("template/page.hbs");
    let _ = h.register_template_string("html", template);
    let html = page.html.as_str();
    let s = h.render("html", &json!({ "content": html, "title": page.title }))?;
    Ok(s)
}

pub fn get_pages_in_dir(dir: &Path, config: &Config) -> Result<Vec<Page>, ContentError> {
    let mut pages: Vec<Page> = Vec::new();

    for entry in read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let meta = metadata(&path)?;
        if meta.is_file() {
            let ext = path.extension();

            if ext.is_some() && ext.unwrap().to_string_lossy().ends_with("md") {
                let page = Page::from_path(&path, config)?;
                pages.push(page);
            }
        }
    }

    if pages.is_empty() {
        return Err(ContentError::NoPages(String::from(dir.to_string_lossy())));
    }

    Ok(pages)
}
