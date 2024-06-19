use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct SectionConfigInput {
    pub(crate) title: Option<String>,
    //pub(crate) order: Option<usize>,
}

pub(crate) struct SectionConfigOutput {
    pub(crate) title: String,
    //pub(crate) order: Option<usize>,
}
