use crate::{
    config::SiteConfig,
    error::Error,
    md::{ast, build_search_index_for_page, render, SearchIndex, TableOfContents},
    utils::get_file,
};

use super::{front::FrontMatter, title::infer_page_title};
use gray_matter::engine::YAML;
use gray_matter::Matter;
use serde::Serialize;
use std::path::{Path, PathBuf};

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

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct Page {
    pub path: String,
    pub relative_path: String,
    pub title: String,
    pub body: String,
    pub html: String,
    pub breadcrumb: Vec<Link>,
    pub table_of_contents: TableOfContents,
    pub search_index: SearchIndex,
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
        let front = FrontMatter::parse(result.data)?;
        let title: String = infer_page_title(front, path, file, &config.title_config);
        let relative_path = path.strip_prefix(&config.root)?.to_string_lossy();
        let tree = ast(&result.content);
        let table_of_contents = TableOfContents::parse(&tree);
        let html = render(&tree);
        let search_index = build_search_index_for_page(&title, &tree);

        Ok(Page {
            path: String::from(path.to_string_lossy()),
            relative_path: String::from(relative_path),
            title,
            body: result.content,
            html,
            breadcrumb: breadcrumb
                .iter()
                .copied()
                .map(|(a, b)| Link::new(a, b))
                .collect(),
            table_of_contents,
            search_index,
        })
    }

    #[cfg(test)]
    pub fn new(
        path: &str,
        relative_path: &str,
        title: &str,
        body: &str,
        html: &str,
        breadcrumb: Vec<Link>,
        table_of_contents: TableOfContents,
        search_index: SearchIndex,
    ) -> Self {
        Self {
            path: String::from(path),
            relative_path: String::from(relative_path),
            title: String::from(title),
            body: String::from(body),
            html: String::from(html),
            breadcrumb,
            table_of_contents,
            search_index,
        }
    }
}
