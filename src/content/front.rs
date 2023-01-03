use gray_matter::Pod;
use serde::Deserialize;

use crate::error::Error;

#[derive(Default, Deserialize)]
pub struct FrontMatter {
    pub title: Option<String>,
}

impl FrontMatter {
    pub fn parse(maybe_pod: Option<Pod>) -> Result<Self, Error> {
        let front: FrontMatter = match maybe_pod {
            Some(f) => f.deserialize()?,
            None => FrontMatter::default(),
        };

        Ok(front)
    }
}
