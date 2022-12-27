use serde::Serialize;

use super::{content::Content, page::Page};

#[derive(Serialize)]
pub struct Site {
    pub content: Content,
}

impl Site {
    pub fn pages(&self) -> Vec<&Page> {
        self.content.pages()
    }
}
