#[cfg(feature = "markdown-it-md")]
mod markdown_it_md;
pub(crate) use markdown_it_md::{
    ast, build_search_index_for_page, get_document_title, render, render_page, SearchDocument,
    SearchIndex, TableOfContents,
};

#[cfg(all(test, feature = "markdown-it-md"))]
pub(crate) use markdown_it_md::TocEntry;
