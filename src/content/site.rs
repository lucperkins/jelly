use serde::Serialize;

use crate::{error::JellyError, md::SearchDocument};

use super::{page::Page, Section};

#[derive(Clone, Debug, PartialEq, Serialize)]
pub(crate) struct Site(pub Section);

#[derive(Clone, Serialize)]
struct PageEntry {
    title: String,
    page_url: String,
}

#[derive(Clone, Serialize)]
pub(crate) struct SiteAttrs {
    title: String,
    pages: Vec<PageEntry>,
    index: String,
}

impl Site {
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

    pub(crate) fn attrs(&self) -> Result<SiteAttrs, JellyError> {
        // TODO: remove unwrap
        let index_json = serde_json::to_string(&self.documents())?;

        Ok(SiteAttrs {
            title: self.0.title.clone(),
            pages: self
                .pages()
                .iter()
                .map(|p| PageEntry {
                    title: p.title.clone(),
                    page_url: p.page_url.clone(),
                })
                .collect(),
            index: index_json,
        })
    }
}
