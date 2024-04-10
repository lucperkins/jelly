use serde::Serialize;

use crate::md::SearchDocument;

use super::{page::Page, Section};

#[derive(Debug, PartialEq, Serialize)]
pub struct Site(pub Section);

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
}
