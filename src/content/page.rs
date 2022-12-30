use crate::{
    config::SiteConfig,
    error::Error,
    md::{ast, build_search_index_for_page, render, SearchIndex, TableOfContents},
    utils::get_file,
};

use super::{breadcrumb::Link, front::FrontMatter, title::infer_page_title};
use gray_matter::engine::YAML;
use gray_matter::Matter;
use serde::Serialize;
use std::{
    cmp::Ordering,
    path::{Path, PathBuf},
};

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
    pub order: Option<usize>,
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

        let order = front.order;

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
            order,
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
        order: Option<usize>,
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
            order,
        }
    }
}

impl Ord for Page {
    fn cmp(&self, other: &Self) -> Ordering {
        let a_ord = self.order.unwrap_or(1);
        let b_ord = other.order.unwrap_or(1);

        if a_ord < b_ord {
            Ordering::Greater
        } else if a_ord == b_ord {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }
}

impl PartialOrd for Page {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
