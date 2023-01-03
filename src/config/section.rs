use serde::Deserialize;

#[derive(Deserialize)]
pub struct SectionConfigInput {
    pub title: Option<String>,
    pub order: Option<usize>,
}

pub struct SectionConfigOutput {
    pub title: String,
    pub order: Option<usize>,
}
