use crate::config::Config;
use crate::error::ContentError;
use handlebars::Handlebars;
use page::Page;
use serde_json::json;
use std::path::{Path, PathBuf};

pub mod config;
pub mod error;
pub mod page;
pub mod section;
pub mod utils;

pub fn build_site(config: &Config) -> Result<(), ContentError> {
    let pages = get_pages(config)?;
    for page in pages {
        let html = render_page(&page)?;
        println!("{}", html);
    }
    Ok(())
}

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
    let md = format!("{}/*.md", dir.display());
    let entries = glob::glob(&md)?;
    for entry in entries {
        let path: PathBuf = entry?;
        let page = Page::from_path(&path, config)?;
        pages.push(page);
    }
    Ok(pages)
}

pub fn get_pages(config: &Config) -> Result<Vec<Page>, ContentError> {
    let mut pages: Vec<Page> = Vec::new();
    let md = format!("{}/**/*.md", config.root.display());
    let entries = glob::glob(&md)?;

    for entry in entries {
        let path: PathBuf = entry?;
        let page = Page::from_path(&path, config)?;
        pages.push(page);
    }

    Ok(pages)
}

mod tests {
    #[test]
    fn example_dir() {
        use super::{get_pages, render_page, Config, Page};
        use indoc::indoc;

        let config = Config::default();
        let pages: Vec<Page> = get_pages(&config).unwrap();
        assert!(!pages[0].id.is_empty());
        assert_eq!(&pages[0].title, "Getting started");
        assert_eq!(&pages[0].path, "docs/getting-started.md");
        assert_eq!(&pages[0].relative_path, "getting-started.md");
        assert_eq!(&pages[0].body, "Here is a getting started thingie.");
        assert_eq!(
            &pages[0].html,
            "<p>Here is a getting started thingie.</p>\n"
        );

        let page = render_page(&pages[0]).unwrap();

        let result = indoc! {"
            <html>
              <head>
                <title>Getting started</title>
              </head>
              <body>
                <div id=\"content\">
                  <p>Here is a getting started thingie.</p>

                </div>
              </body>
            </html>
        "};

        assert_eq!(&page, result);

        assert!(!pages[1].id.is_empty());
        assert_eq!(&pages[1].title, "Welcome");
        assert_eq!(&pages[1].path, "docs/index.md");
        assert_eq!(&pages[1].relative_path, "index.md");
        assert_eq!(
            &pages[1].body,
            "Here is some content.\n\n## Heading\n\nAnd some more."
        );
        assert_eq!(
            &pages[1].html,
            "<p>Here is some content.</p>\n<h2>Heading</h2>\n<p>And some more.</p>\n"
        );
    }
}
