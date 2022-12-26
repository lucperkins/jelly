use crate::{
    front::FrontMatter,
    md::{ast, render, TableOfContents},
    title::infer_page_title,
};

use super::config::SiteConfig;
use super::error::Error;
use super::utils::get_file;
use gray_matter::engine::YAML;
use gray_matter::Matter;
use serde::Serialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize)]
pub struct Link {
    path: PathBuf,
    title: String,
}

#[derive(Debug, Serialize)]
pub struct Page {
    pub path: String,
    pub relative_path: String,
    pub title: String,
    pub body: String,
    pub html: String,
    pub breadcrumb: Vec<Link>,
    pub table_of_contents: TableOfContents,
}

impl Page {
    pub fn from_path(
        path: &Path,
        breadcrumb: &[(&PathBuf, &str)],
        config: &SiteConfig,
    ) -> Result<Self, Error> {
        let file = get_file(path)?;

        let matter = Matter::<YAML>::new();
        let result = matter.parse(&file);

        let front: FrontMatter = match result.data {
            Some(f) => f.deserialize()?,
            None => FrontMatter::default(),
        };

        let title: String = infer_page_title(front, path, file, &config.title_config);

        let relative_path = path.strip_prefix(&config.root)?.to_string_lossy();

        let tree = ast(&result.content);

        let table_of_contents = TableOfContents::new(&tree);
        let html = render(&tree);

        Ok(Page {
            path: String::from(path.to_string_lossy()),
            relative_path: String::from(relative_path),
            title,
            body: result.content,
            html,
            breadcrumb: breadcrumb
                .iter()
                .copied()
                .map(|(a, b)| Link {
                    path: PathBuf::from(a),
                    title: String::from(b),
                })
                .collect(),
            table_of_contents,
        })
    }
}
