use serde::Serialize;

use super::{page::Page, section::Content};

#[derive(Serialize)]
pub struct Site {
    pub content: Content,
}

impl Site {
    pub fn pages(&self) -> Vec<&Page> {
        self.content.pages()
    }
}
