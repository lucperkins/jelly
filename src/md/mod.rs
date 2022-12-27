mod code;
mod highlight;
mod parse;
mod title;

pub use parse::{ast, node_to_string, render};
pub use title::get_document_title;
