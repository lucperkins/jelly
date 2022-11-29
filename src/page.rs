use super::config::Config;
use super::error::ContentError;
use super::utils::{get_file, name_from_path};
use comrak::{markdown_to_html, ComrakOptions};
use gray_matter::engine::YAML;
use gray_matter::Matter;
use serde::Deserialize;
use std::path::Path;

pub struct Page {
    pub id: String,
    pub path: String,
    pub relative_path: String,
    pub title: String,
    pub body: String,
    pub html: String,
}

impl Page {
    pub fn from_path(path: &Path, config: &Config) -> Result<Self, ContentError> {
        let file = get_file(path)?;

        let matter = Matter::<YAML>::new();
        let result = matter.parse(&file);

        let front: FrontMatter = match result.data {
            Some(f) => f.deserialize()?,
            None => FrontMatter::default(),
        };

        let title: String = infer_page_title(front, path, &config.title_config);

        let id = base64::encode(&title);

        let relative_path = path.strip_prefix(&config.root)?.to_string_lossy();

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

pub struct TitleConfig {
    pub title_case: bool,
    pub first_letter_capitalized: bool,
}

impl Default for TitleConfig {
    fn default() -> Self {
        Self {
            title_case: false,
            first_letter_capitalized: true,
        }
    }
}

fn infer_page_title(front: FrontMatter, path: &Path, title_config: &TitleConfig) -> String {
    name_from_path(front.title, path, title_config)
}
