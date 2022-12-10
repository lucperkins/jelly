use std::path::PathBuf;

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

pub struct TitleConfig {
    pub title_case: bool,
    pub first_letter_capitalized: bool,
}

impl Default for TitleConfig {
    fn default() -> Self {
        Self {
            title_case: false,
            first_letter_capitalized: true,
        }
    }
}
