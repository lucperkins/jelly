#[derive(thiserror::Error, Debug)]
pub enum ContentError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("pattern error: {0}")]
    Pattern(#[from] glob::PatternError),

    #[error("glob error: {0}")]
    Glob(#[from] glob::GlobError),

    #[error("prefix error: {0}")]
    Prefix(#[from] std::path::StripPrefixError),

    #[error("render error: {0}")]
    Render(#[from] handlebars::RenderError),

    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("template error: {0}")]
    Template(#[from] handlebars::TemplateError),
}