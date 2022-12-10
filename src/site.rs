use serde::Serialize;

use crate::config::SiteConfig;
use crate::content::{get_section, Content};
use crate::error::ContentError;

#[derive(Serialize)]
pub struct Site {
    pub content: Content,
}

pub fn build_site(config: &SiteConfig) -> Result<(), ContentError> {
    let content = get_section(&config.root, config)?;

    let site = Site { content };

    let site_json_str = serde_json::to_string_pretty(&site)?;

    println!("{}", site_json_str);

    Ok(())
}
