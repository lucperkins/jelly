use std::path::PathBuf;

use serde::Serialize;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct Link {
    pub(crate) path: PathBuf,
    pub(crate) title: String,
}

impl Link {
    pub(crate) fn new(path: &PathBuf, title: &str) -> Self {
        Self {
            path: PathBuf::from(path),
            title: String::from(title),
        }
    }
}
