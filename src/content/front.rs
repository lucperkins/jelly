use gray_matter::Pod;
use serde::Deserialize;

use crate::error::JellyError;

#[derive(Default, Deserialize)]
pub(super) struct FrontMatter {
    pub(super) title: Option<String>,
    pub(super) order: Option<usize>,
}

impl FrontMatter {
    pub(super) fn parse(maybe_pod: Option<Pod>) -> Result<Self, JellyError> {
        Ok(match maybe_pod {
            Some(f) => f.deserialize()?,
            None => FrontMatter::default(),
        })
    }
}
