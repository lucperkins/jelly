use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("glob error: {0}")]
    Glob(#[from] glob::GlobError),

    #[error("highlight error: {0}")]
    Highlight(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("no _meta.yaml found in directory: {0}")]
    NoMetaYamlFile(String),

    #[error("no pages found in directory: {0}")]
    NoPages(String),

    #[error("pattern error: {0}")]
    Pattern(#[from] glob::PatternError),

    #[error("prefix error: {0}")]
    Prefix(#[from] std::path::StripPrefixError),

    #[error("render error: {0}")]
    Render(#[from] handlebars::RenderError),

    #[error("syntect error: {0}")]
    Syntect(#[from] syntect::Error),

    #[error("template error: {0}")]
    Template(#[from] Box<handlebars::TemplateError>),

    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("yaml parse error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("order parameter on page {0} is set to zero")]
    ZeroOrder(PathBuf),
}
