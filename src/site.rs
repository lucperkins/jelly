use crate::config::Config;
use crate::error::ContentError;
use crate::section::{get_sections, Section};

pub struct Site {
    pub sections: Vec<Section>,
}

pub fn build_site(config: &Config) -> Result<Site, ContentError> {
    let sections = get_sections(&config.root, config)?;

    let site = Site { sections };

    Ok(site)
}
