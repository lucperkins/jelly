mod code;
mod headings;
mod highlight;
mod image;
mod parse;
mod render;
mod search;
mod title;
mod toc;

pub(crate) use parse::{ast, render};
pub(crate) use render::render_page;
pub(crate) use search::{build_search_index_for_page, SearchDocument, SearchIndex};
pub(crate) use title::get_document_title;
pub(crate) use toc::TableOfContents;
#[cfg(test)]
pub(crate) use toc::TocEntry;
