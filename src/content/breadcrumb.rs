use std::path::PathBuf;

use serde::Serialize;

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct Link {
    pub path: PathBuf,
    pub title: String,
}

impl Link {
    pub fn new(path: &PathBuf, title: &str) -> Self {
        Self {
            path: PathBuf::from(path),
            title: String::from(title),
        }
    }
}
