use crate::config::SiteConfig;
use crate::error::ContentError;
use crate::get_pages_in_dir;
use crate::md::get_document_title;
use crate::page::Page;
use crate::utils::{get_file, get_or_none, name_from_path};
use serde::{Deserialize, Serialize};
use std::fs::{metadata, read_dir, read_to_string};
use std::path::Path;

pub type Content = Section;

#[derive(Deserialize)]
struct SectionConfig {
    title: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Section {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<Vec<Page>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sections: Option<Vec<Section>>,
}

pub fn get_section(path: &Path, config: &SiteConfig) -> Result<Section, ContentError> {
    let root_section_title = get_section_title(path, config)?;
    let pages: Vec<Page> = get_pages_in_dir(path, config)?;
    let mut sections: Vec<Section> = Vec::new();

    for entry in read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let meta = metadata(&path)?;

        if meta.is_dir() {
            let section = get_section(&path, config)?;
            sections.push(section);
        }
    }

    let root_section = Section {
        title: root_section_title,
        pages: get_or_none(pages),
        sections: get_or_none(sections),
    };

    Ok(root_section)
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

fn get_section_title(path: &Path, config: &SiteConfig) -> Result<String, ContentError> {
    let title: String;
    let yaml_file_path = Path::new(&path).join("_dir.yaml");
    if yaml_file_path.exists() {
        let yaml_file_str = read_to_string(&yaml_file_path)?;
        let section_config: SectionConfig = serde_yaml::from_str(&yaml_file_str)?;
        match section_config.title {
            Some(t) => title = t,
            None => {
                let t = title_from_index_page(path)?;
                title = t.unwrap_or_else(|| name_from_path(path, &config.title_config))
            }
        }
    } else {
        let t = title_from_index_page(path)?;
        title = t.unwrap_or_else(|| name_from_path(path, &config.title_config))
    };
    Ok(title)
}
