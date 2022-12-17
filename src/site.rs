use crate::content::Content;
use crate::page::Page;
use serde::Serialize;

#[derive(Serialize)]
pub struct Site {
    pub content: Content,
}

impl Site {
    pub fn pages(&self) -> Vec<&Page> {
        self.content.pages()
    }
}
