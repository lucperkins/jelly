use comrak::{markdown_to_html, ComrakOptions};
use gray_matter::engine::YAML;
use gray_matter::Matter;
use handlebars::{Handlebars, RenderError, TemplateError};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::string::FromUtf8Error;
use titlecase::titlecase;

fn render_page(page: &Page) -> Result<String, ContentError> {
    let mut h = Handlebars::new();
    h.register_template_file("html", "src/template/page.hbs")?;
    let html = page.html.as_str();
    let s = h.render("html", &json!({ "content": html, "title": page.title }))?;
    Ok(s)
}

#[derive(Serialize)]
pub struct Page {
    pub id: String,
    pub path: String,
    pub relative_path: String,
    pub title: String,
    pub body: String,
    pub html: String,
}

impl Page {
    fn from_path(path: &Path, config: &Config) -> Result<Self, ContentError> {
        let mut file = File::open(path.as_os_str())?;
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents)?;

        let matter = Matter::<YAML>::new();
        let result = matter.parse(&contents);

        let front: FrontMatter = match result.data {
            Some(f) => f.deserialize()?,
            None => FrontMatter::default(),
        };

        let title: String = infer_title(front, path, &config.title_config);

        let id = base64::encode(&title);

        let relative_path = path.strip_prefix(config.root)?.to_string_lossy();

        let options = ComrakOptions::default();

        let html = markdown_to_html(&result.content, &options);

        Ok(Page {
            id,
            path: String::from(path.to_string_lossy()),
            relative_path: String::from(relative_path),
            title,
            body: result.content,
            html,
        })
    }
}

#[derive(Default, Deserialize)]
struct FrontMatter {
    title: Option<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum ContentError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("pattern error: {0}")]
    Pattern(#[from] glob::PatternError),

    #[error("glob error: {0}")]
    Glob(#[from] glob::GlobError),

    #[error("prefix error: {0}")]
    Prefix(#[from] std::path::StripPrefixError),

    #[error("render error: {0}")]
    Render(#[from] RenderError),

    #[error("UTF-8 error: {0}")]
    Utf8(#[from] FromUtf8Error),

    #[error("template error: {0}")]
    Template(#[from] TemplateError),
}

pub struct Config {
    root: &'static str,
    title_config: TitleConfig,
}

struct TitleConfig {
    title_case: bool,
    first_letter_capitalized: bool,
}

impl Default for TitleConfig {
    fn default() -> Self {
        Self {
            title_case: false,
            first_letter_capitalized: true,
        }
    }
}

fn infer_title(front: FrontMatter, path: &Path, title_config: &TitleConfig) -> String {
    front.title.unwrap_or_else(|| {
        let stem = path.file_stem().unwrap();

        let deslugged = stem.to_string_lossy().replace("-", " ");

        if title_config.title_case {
            titlecase(&deslugged)
        } else if title_config.first_letter_capitalized {
            capitalize_first_letter(&deslugged)
        } else {
            deslugged
        }
    })
}

fn capitalize_first_letter(s: &str) -> String {
    s[0..1].to_uppercase() + &s[1..]
}

pub fn get_pages(config: Config) -> Result<Vec<Page>, ContentError> {
    let mut pages: Vec<Page> = Vec::new();
    let md = format!("{}/**/*.md", config.root);
    let entries = glob::glob(&md)?;

    for entry in entries {
        let path: PathBuf = entry?;
        let page = Page::from_path(&path, &config)?;
        pages.push(page);
    }

    Ok(pages)
}

mod tests {
    #[test]
    fn example_dir() {
        use super::{get_pages, render_page, Config, Page, TitleConfig};
        use indoc::indoc;

        let config = Config {
            root: "example",
            title_config: TitleConfig::default(),
        };
        let pages: Vec<Page> = get_pages(config).unwrap();
        assert!(!pages[0].id.is_empty());
        assert_eq!(&pages[0].title, "Getting started");
        assert_eq!(&pages[0].path, "example/getting-started.md");
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
        assert_eq!(&pages[1].path, "example/index.md");
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
