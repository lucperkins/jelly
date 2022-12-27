use serde::Serialize;

use super::{page::Page, Section};

#[derive(Debug, PartialEq, Serialize)]
pub struct Site {
    pub content: Section,
}

impl Site {
    pub fn pages(&self) -> Vec<&Page> {
        self.content.pages()
    }
}
