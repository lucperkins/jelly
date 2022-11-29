use super::config::Config;
use super::error::ContentError;
use comrak::{markdown_to_html, ComrakOptions};
use gray_matter::engine::YAML;
use gray_matter::Matter;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use titlecase::titlecase;

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

        #[allow(clippy::single_char_pattern)]
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
