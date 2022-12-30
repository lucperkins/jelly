use serde::Deserialize;

#[derive(Default, Deserialize)]
pub struct FrontMatter {
    pub title: Option<String>,
    pub order: Option<usize>,
}
