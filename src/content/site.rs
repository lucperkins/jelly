use serde::Serialize;

use crate::md::SearchDocument;

use super::{page::Page, Section};

#[derive(Debug, PartialEq, Serialize)]
pub struct Site(pub Section);

#[derive(Serialize)]
struct PageEntry {
    title: String,
    page_url: String,
}

#[derive(Serialize)]
pub struct SiteAttrs<'a> {
    title: &'a str,
    pages: Vec<PageEntry>,
    index: String,
}

impl Site {
    pub fn pages(&self) -> Vec<&Page> {
        self.0.pages()
    }

    pub fn documents(&self) -> Vec<SearchDocument> {
        let mut docs: Vec<SearchDocument> = Vec::new();

        for page in self.pages() {
            let documents = page.search_index.0.clone();
            for doc in documents {
                docs.push(doc);
            }
        }

        docs
    }

    pub fn attrs(&self) -> SiteAttrs {
        // TODO: remove unwrap
        let index_json = serde_json::to_string(&self.documents()).unwrap();

        SiteAttrs {
            title: &self.0.title,
            pages: self
                .pages()
                .iter()
                .map(|p| PageEntry {
                    title: p.title.clone(),
                    page_url: p.page_url.clone(),
                })
                .collect(),
            index: index_json,
        }
    }
}
