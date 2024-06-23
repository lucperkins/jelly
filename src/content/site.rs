use std::{fs::create_dir_all, path::PathBuf};

use ammonia::clean;
use serde::Serialize;

use crate::{
    config::SiteConfig,
    error::JellyError,
    md::{render_page, SearchDocument},
    utils::write_file,
};

use super::{page::Page, section::SectionEntry, Section};

#[derive(Clone, Debug, PartialEq, Serialize)]
pub(crate) struct Site(pub(crate) Section);

#[derive(Clone, Serialize)]
pub(crate) struct SiteAttrs {
    title: String,
    sections: Option<Vec<SectionEntry>>,
}

#[derive(Serialize)]
pub(crate) struct SiteIndex(Vec<SearchDocument>);

impl Site {
    pub(crate) fn write(
        config: &SiteConfig,
        out: &PathBuf,
        sanitize: bool,
    ) -> Result<(), JellyError> {
        let this: Self = Self::build(&config)?;

        for page in this.pages() {
            let html = render_page(page, &this.attrs())?;
            let mut path = page.html_path(&out);

            if let Some(dir) = path.as_path().parent() {
                create_dir_all(dir)?;
            }

            let final_html = if sanitize { clean(&html) } else { html };

            path.set_extension("html");

            write_file(&path, final_html)?;
        }

        Ok(())
    }

    pub(crate) fn build(config: &SiteConfig) -> Result<Self, JellyError> {
        Ok(Self(Section::from_path(&config.root, None, &config)?))
    }

    pub(crate) fn index(&self) -> SiteIndex {
        SiteIndex(self.documents())
    }

    pub(crate) fn pages(&self) -> Vec<&Page> {
        self.0.pages()
    }

    pub(crate) fn documents(&self) -> Vec<SearchDocument> {
        let mut docs: Vec<SearchDocument> = Vec::new();

        for page in self.pages() {
            let documents = page.search_index.0.clone();
            for doc in documents {
                docs.push(doc);
            }
        }

        docs
    }

    pub(crate) fn attrs(&self) -> SiteAttrs {
        SiteAttrs {
            title: self.0.title.clone(),
            sections: self
                .0
                .sections
                .as_ref()
                .map(|ss| ss.iter().map(SectionEntry::from).collect()),
        }
    }
}
