use crate::{
    config::SiteConfig,
    error::JellyError,
    md::{ast, build_search_index_for_page, render, SearchIndex, TableOfContents},
    utils::get_file,
};

use super::{
    breadcrumb::Link,
    front::FrontMatter,
    title::{infer_page_title, WithTitle},
};
use gray_matter::{engine::YAML, Matter};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::Serialize;
use std::{
    cmp::Ordering,
    path::{Path, PathBuf},
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct Page {
    pub(crate) path: String,
    pub(crate) relative_path: String,
    pub(crate) url: String,
    pub(crate) title: String,
    pub(crate) body: String,
    pub(crate) html: String,
    pub(crate) breadcrumb: Vec<Link>,
    pub(crate) table_of_contents: TableOfContents,
    pub(crate) search_index: SearchIndex,
    pub(crate) order: Option<usize>,
}

#[derive(Clone, Serialize)]
pub(super) struct PageEntry {
    pub(super) title: String,
    pub(super) url: String,
}

impl Page {
    pub(super) fn from_path(
        path: &Path,
        breadcrumb: &[(&PathBuf, &str)],
        config: &SiteConfig,
    ) -> Result<Self, JellyError> {
        let file: String = get_file(path)?;
        let matter = Matter::<YAML>::new();
        let result = matter.parse(&file);
        let front = FrontMatter::parse(result.data)?;
        let order = front.order;

        if let Some(order) = order {
            if order == 0 {
                return Err(JellyError::ZeroOrder(path.to_path_buf()));
            }
        }

        let title: String = infer_page_title(front, path, file, &config.title_config);
        let relative_path = path.strip_prefix(&config.root)?;
        let tree = ast(&result.content);
        let table_of_contents = TableOfContents::parse(&tree);
        let html = render(&tree);
        let search_index = build_search_index_for_page(&title, &tree);

        let url = if let Some(last_segment) = relative_path
            .with_extension("")
            .file_name()
            .and_then(|name| name.to_str())
        {
            if last_segment == "index" {
                relative_path
                    .parent()
                    .unwrap_or(relative_path)
                    .to_path_buf()
            } else {
                relative_path.to_path_buf()
            }
        } else {
            relative_path.to_path_buf()
        };

        Ok(Page {
            path: String::from(path.to_string_lossy()),
            relative_path: String::from(relative_path.to_string_lossy()),
            url: String::from(url.to_string_lossy()),
            title,
            body: result.content,
            html,
            breadcrumb: breadcrumb
                .par_iter()
                .copied()
                .map(|(a, b)| Link::new(a, b))
                .collect(),
            table_of_contents,
            search_index,
            order,
        })
    }

    pub(crate) fn html_path(&self, root: PathBuf) -> PathBuf {
        root.join(&self.relative_path)
    }

    fn is_index(&self) -> bool {
        self.relative_path == "index.md"
    }

    #[allow(clippy::too_many_arguments)]
    #[cfg(test)]
    pub(crate) fn new(
        path: &str,
        relative_path: &str,
        url: &str,
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
            url: String::from(url),
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
        let a_ord = if self.is_index() {
            0
        } else {
            self.order.unwrap_or(1)
        };
        let b_ord = if other.is_index() {
            0
        } else {
            other.order.unwrap_or(1)
        };

        a_ord.cmp(&b_ord).reverse()
    }
}

impl PartialOrd for Page {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl WithTitle for Page {
    fn title(&self) -> String {
        self.title.to_owned()
    }
}
