#[derive(thiserror::Error, Debug)]
pub enum ContentError {
    #[error("glob error: {0}")]
    Glob(#[from] glob::GlobError),

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

    #[error("template error: {0}")]
    Template(#[from] handlebars::TemplateError),

    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("walk dir error: {0}")]
    Walk(#[from] walkdir::Error),

    #[error("yaml parse error: {0}")]
    Yaml(#[from] serde_yaml::Error),
}
