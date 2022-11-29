use crate::page::TitleConfig;
use std::path::PathBuf;

static DEFAULT_DOCS_DIR: &str = "docs";

pub struct Config {
    pub root: PathBuf,
    pub title_config: TitleConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            root: PathBuf::from(DEFAULT_DOCS_DIR),
            title_config: TitleConfig::default(),
        }
    }
}
