use crate::config::Config;
use crate::error::ContentError;
use crate::get_pages_in_dir;
use crate::md::get_document_title;
use crate::page::Page;
use crate::utils::{get_file, name_from_path};
use serde::Deserialize;
use std::fs::{metadata, read_dir, read_to_string};
use std::path::Path;

#[derive(Deserialize)]
struct SectionConfig {
    title: Option<String>,
}

#[derive(Debug)]
pub struct Section {
    pub title: String,
    pub pages: Vec<Page>,
    pub sections: Option<Vec<Section>>,
}

pub fn get_sections(path: &Path, config: &Config) -> Result<Vec<Section>, ContentError> {
    let mut sections: Vec<Section> = Vec::new();

    for entry in read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let meta = metadata(&path)?;

        if meta.is_dir() {
            let section = dir_to_section(&path, config)?;
            sections.push(section);
        }
    }

    Ok(sections)
}

fn title_from_index_page(path: &Path) -> Result<Option<String>, ContentError> {
    let index_path = Path::new(&path).join("index.md");
    if index_path.exists() {
        let file = get_file(path)?;
        match get_document_title(&file) {
            Some(t) => Ok(Some(t)),
            None => Ok(None),
        }
    } else {
        Ok(None)
    }
}

fn dir_to_section(path: &Path, config: &Config) -> Result<Section, ContentError> {
    let yaml_file_path = Path::new(&path).join("_dir.yaml");

    let title: String;

    // Check if _dir.yaml exists
    if yaml_file_path.exists() {
        let yaml_file_str = read_to_string(&yaml_file_path)?;
        let section_config: SectionConfig = serde_yaml::from_str(&yaml_file_str)?;
        match section_config.title {
            Some(t) => {
                title = t;
            }
            None => {
                let t = title_from_index_page(path)?;
                title = t.unwrap_or_else(|| name_from_path(path, &config.title_config))
            }
        }
    } else {
        let t = title_from_index_page(path)?;
        title = t.unwrap_or_else(|| name_from_path(path, &config.title_config));
    }

    let pages = get_pages_in_dir(path, config)?;

    let sub_sections = get_sections(path, config)?;
    let sub_sections = if sub_sections.is_empty() {
        None
    } else {
        Some(sub_sections)
    };

    Ok(Section {
        title,
        pages,
        sections: sub_sections,
    })
}
