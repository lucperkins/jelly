use serde::Serialize;

use crate::md::SearchDocument;

use super::{page::Page, Section};

#[derive(Debug, PartialEq, Serialize)]
pub struct Site(pub Section);

#[derive(Serialize)]
struct PageEntry {
    title: String,
    path: String,
}

#[derive(Serialize)]
pub struct SiteAttrs<'a> {
    title: &'a str,
    pages: Vec<PageEntry>,
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
        SiteAttrs {
            title: &self.0.title,
            pages: self
                .pages()
                .iter()
                .map(|p| PageEntry {
                    title: p.title.clone(),
                    path: p.path.clone(),
                })
                .collect(),
        }
    }
}
