use std::path::PathBuf;

use super::TitleConfig;

static DEFAULT_DOCS_DIR: &str = "docs";

pub struct SiteConfig {
    pub root: PathBuf,
    pub title_config: TitleConfig,
}

impl Default for SiteConfig {
    fn default() -> Self {
        Self {
            root: PathBuf::from(DEFAULT_DOCS_DIR),
            title_config: TitleConfig::default(),
        }
    }
}
