use std::path::PathBuf;

use super::TitleConfig;

const DEFAULT_DOCS_DIR: &str = "docs";

pub(crate) struct SiteConfig {
    pub(crate) root: PathBuf,
    pub(crate) title_config: TitleConfig,
}

impl SiteConfig {
    pub(crate) fn new(root: PathBuf) -> Self {
        Self {
            root: root.to_path_buf(),
            title_config: TitleConfig::default(),
        }
    }
}

impl Default for SiteConfig {
    fn default() -> Self {
        Self {
            root: PathBuf::from(DEFAULT_DOCS_DIR),
            title_config: TitleConfig::default(),
        }
    }
}
