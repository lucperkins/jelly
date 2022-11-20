use glob::glob;
use gray_matter::engine::YAML;
use gray_matter::Matter;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use titlecase::titlecase;

#[derive(Serialize)]
struct Page {
    id: String,
    path: String,
    title: String,
    body: String,
}

#[derive(Deserialize)]
struct FrontMatter {
    title: Option<String>,
}

impl Default for FrontMatter {
    fn default() -> Self {
        Self { title: None }
    }
}

#[derive(thiserror::Error, Debug)]
enum ContentError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("error parsing front matter")]
    FrontMatter,

    #[error("pattern error: {0}")]
    Pattern(#[from] glob::PatternError),

    #[error("glob error: {0}")]
    Glob(#[from] glob::GlobError),
}

struct Config {
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

impl Page {
    fn from_path(path: &Path, title_config: &TitleConfig) -> Result<Self, ContentError> {
        let mut file = File::open(path.as_os_str())?;
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents)?;

        let matter = Matter::<YAML>::new();
        let result = matter.parse(&contents);

        let front: FrontMatter = match result.data {
            Some(f) => f.deserialize()?,
            None => FrontMatter::default(),
        };

        let path_str = &String::from(path.to_string_lossy());

        let title: String = front.title.unwrap_or_else(|| {
            let stem = path.file_stem().unwrap();
            let deslugged = stem.to_string_lossy().replace("-", " ");

            if title_config.title_case {
                titlecase(&deslugged)
            } else {
                if title_config.first_letter_capitalized {
                    capitalize_first_letter(&deslugged)
                } else {
                    deslugged
                }
            }
        });

        let id = base64::encode(&title);

        Ok(Page {
            id,
            path: String::from(path_str),
            title,
            body: result.content,
        })
    }
}

fn capitalize_first_letter(s: &str) -> String {
    s[0..1].to_uppercase() + &s[1..]
}

fn get_pages(config: Config) -> Result<Vec<Page>, ContentError> {
    let mut pages: Vec<Page> = Vec::new();
    let md = format!("{}/**/*.md", config.root);
    let entries = glob::glob(&md)?;

    for entry in entries {
        let path = entry?;
        let page = Page::from_path(&path, &config.title_config)?;
        pages.push(page);
    }

    Ok(pages)
}

fn main() {
    let config = Config {
        root: "example",
        title_config: TitleConfig::default(),
    };

    let pages: Vec<Page> = get_pages(config).unwrap();

    for page in pages {
        println!(
            "(\n  id: {id}\n  path: {path}\n  title: {title}\n  body: {body}\n)",
            id = page.id,
            path = page.path,
            title = page.title,
            body = page.body
        );
    }
}
