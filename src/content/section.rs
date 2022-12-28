use crate::config::SiteConfig;
use crate::error::Error;
use crate::utils::get_or_none;
use serde::Serialize;
use std::fs::{metadata, read_dir};
use std::path::PathBuf;

use super::page::Page;
use super::title::get_section_title;

#[derive(Debug, PartialEq, Serialize)]
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

    pub fn from_path(
        path: &PathBuf,
        breadcrumb: Option<&Vec<(&PathBuf, &str)>>,
        config: &SiteConfig,
    ) -> Result<Self, Error> {
        let section_title = &get_section_title(path, config)?;
        let mut breadcrumb_acc: Vec<(&PathBuf, &str)> = Vec::new();

        if let Some(bc) = breadcrumb {
            for t in bc {
                breadcrumb_acc.push(*t);
            }
        }

        breadcrumb_acc.push((path, section_title));

        let pages: Vec<Page> = get_pages_in_dir(path, &breadcrumb_acc, config)?;
        let mut sections: Vec<Section> = Vec::new();

        for entry in read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            let meta = metadata(&path)?;

            if meta.is_dir() {
                let section = Self::from_path(&path, Some(&breadcrumb_acc), config)?;
                sections.push(section);
            }
        }

        Ok(Section {
            title: String::from(section_title),
            pages: get_or_none(pages),
            sections: get_or_none(sections),
        })
    }
}

fn get_pages_in_dir(
    dir: &PathBuf,
    breadcrumb: &[(&PathBuf, &str)],
    config: &SiteConfig,
) -> Result<Vec<Page>, Error> {
    let mut pages: Vec<Page> = Vec::new();

    for entry in read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let meta = metadata(&path)?;
        if meta.is_file() {
            let ext = path.extension();

            if ext.is_some() && ext.unwrap().to_string_lossy().ends_with("md") {
                let page = Page::from_path(&path, breadcrumb, config)?;
                pages.push(page);
            }
        }
    }

    if pages.is_empty() {
        return Err(Error::NoPages(String::from(dir.to_string_lossy())));
    }

    Ok(pages)
}