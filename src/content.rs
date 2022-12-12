use crate::config::SiteConfig;
use crate::error::ContentError;
use crate::get_pages_in_dir;
use crate::page::Page;
use crate::title::get_section_title;
use crate::utils::get_or_none;
use serde::{Deserialize, Serialize};
use std::fs::{metadata, read_dir};
use std::path::Path;

pub type Content = Section;

#[derive(Deserialize)]
pub struct SectionConfig {
    pub title: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Section {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<Vec<Page>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sections: Option<Vec<Section>>,
}

impl Section {
    pub fn pages(&self) -> Vec<&Page> {
        let mut pages: Vec<&Page> = Vec::new();

        if let Some(section_pages) = &self.pages {
            for page in section_pages {
                pages.push(page);
            }
        }

        if let Some(sections) = &self.sections {
            for section in sections {
                let section_pages = Self::pages(section);
                for page in section_pages {
                    pages.push(page);
                }
            }
        }

        pages
    }

    pub fn from_path(path: &Path, config: &SiteConfig) -> Result<Self, ContentError> {
        let root_section_title = get_section_title(path, config)?;
        let pages: Vec<Page> = get_pages_in_dir(path, config)?;
        let mut sections: Vec<Section> = Vec::new();

        for entry in read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            let meta = metadata(&path)?;

            if meta.is_dir() {
                let section = Self::from_path(&path, config)?;
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
}
