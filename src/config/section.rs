use serde::Deserialize;

#[derive(Deserialize)]
pub struct SectionConfig {
    pub title: Option<String>,
}
