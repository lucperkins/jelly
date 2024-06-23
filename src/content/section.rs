use crate::config::SiteConfig;
use crate::error::JellyError;
use crate::utils::get_or_none;
use serde::Serialize;
use std::fs::{metadata, read_dir};
use std::path::PathBuf;

use super::by_title;
use super::page::{Page, PageEntry};
use super::title::{get_section_config, WithTitle};

#[derive(Clone, Debug, PartialEq, Serialize)]
pub(crate) struct Section {
    pub(super) title: String,
    pub(super) url: String,
    pub(super) pages: Option<Vec<Page>>,
    pub(super) sections: Option<Vec<Section>>,
}

#[derive(Clone, Serialize)]
pub(super) struct SectionEntry {
    pub(super) title: String,
    pub(super) url: String,
    pub(super) pages: Option<Vec<PageEntry>>,
    pub(super) sections: Option<Vec<SectionEntry>>,
}

impl From<&Section> for SectionEntry {
    fn from(s: &Section) -> Self {
        Self {
            title: s.title.clone(),
            url: s.url.clone(),
            pages: s.pages.as_ref().map(|ps| {
                ps.iter()
                    .map(|Page { title, url, .. }| PageEntry {
                        title: title.to_string(),
                        url: url.to_string(),
                    })
                    .collect()
            }),
            sections: s
                .sections
                .as_ref()
                .map(|ss| ss.iter().map(Self::from).collect()),
        }
    }
}

impl Section {
    pub(crate) fn pages(&self) -> Vec<&Page> {
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

        pages.sort();
        pages.sort_by(by_title);

        pages
    }

    pub(crate) fn from_path(
        path: &PathBuf,
        breadcrumb: Option<&Vec<(&PathBuf, &str)>>,
        config: &SiteConfig,
    ) -> Result<Self, JellyError> {
        let section_config = &get_section_config(path, config)?;
        let title = section_config.title.clone();

        let mut breadcrumb_acc: Vec<(&PathBuf, &str)> = Vec::new();

        if let Some(bc) = breadcrumb {
            for t in bc {
                breadcrumb_acc.push(*t);
            }
        }

        breadcrumb_acc.push((path, &section_config.title));

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
            title,
            url: path.display().to_string(),
            pages: get_or_none(pages),
            sections: get_or_none(sections),
        })
    }

    #[cfg(test)]
    pub(crate) fn new(
        title: &str,
        url: &str,
        pages: Option<Vec<Page>>,
        sections: Option<Vec<Section>>,
    ) -> Self {
        Self {
            title: String::from(title),
            url: String::from(url),
            pages,
            sections,
        }
    }
}

impl WithTitle for Section {
    fn title(&self) -> String {
        self.title.to_owned()
    }
}

fn get_pages_in_dir(
    dir: &PathBuf,
    breadcrumb: &[(&PathBuf, &str)],
    config: &SiteConfig,
) -> Result<Vec<Page>, JellyError> {
    let mut pages: Vec<Page> = Vec::new();

    for entry in read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let meta = metadata(&path)?;
        if meta.is_file() {
            if let Some(ext) = path.extension() {
                if ext.to_string_lossy().ends_with("md") {
                    let page = Page::from_path(&path, breadcrumb, config)?;
                    pages.push(page);
                }
            }
        }
    }

    if pages.is_empty() {
        return Err(JellyError::NoPages(String::from(dir.to_string_lossy())));
    }

    Ok(pages)
}
