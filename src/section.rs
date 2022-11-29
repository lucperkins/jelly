use crate::config::Config;
use crate::error::ContentError;
use crate::get_pages_in_dir;
use crate::page::{Page, TitleConfig};
use serde::Deserialize;
use std::fs::read_to_string;
use std::path::Path;
use titlecase::titlecase;
use walkdir::WalkDir;

#[derive(Deserialize)]
struct SectionConfig {
    title: Option<String>,
}

pub struct Section {
    pub title: String,
    pub pages: Vec<Page>,
}

fn infer_section_title(
    section_config: SectionConfig,
    path: &Path,
    title_config: &TitleConfig,
) -> String {
    section_config.title.unwrap_or_else(|| {
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

pub fn get_sections(config: &Config) -> Result<Vec<Section>, ContentError> {
    let mut sections: Vec<Section> = Vec::new();

    for entry in WalkDir::new(&config.root) {
        let maybe_entry = entry?;
        let path = maybe_entry.path();

        if path.is_dir() {
            let yaml_path = Path::new(path).join("_meta.yaml");
            if yaml_path.exists() {
                let yaml_file_str = read_to_string(yaml_path)?;
                let section_config: SectionConfig = serde_yaml::from_str(&yaml_file_str)?;
                let section_title =
                    infer_section_title(section_config, &path, &config.title_config);

                let pages = get_pages_in_dir(path, config)?;

                if pages.len() == 0 {
                    return Err(ContentError::NoPages(String::from(path.to_string_lossy())));
                }

                let section = Section {
                    title: section_title,
                    pages,
                };
                sections.push(section);
            }
        }
    }

    Ok(sections)
}
