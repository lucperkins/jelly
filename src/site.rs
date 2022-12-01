use serde::Serialize;

use crate::config::Config;
use crate::content::{get_section, Content};
use crate::error::ContentError;

#[derive(Serialize)]
pub struct Site {
    pub content: Content,
}

pub fn build_site(config: &Config) -> Result<Site, ContentError> {
    let content = get_section(&config.root, config)?;

    let site = Site { content };

    Ok(site)
}
