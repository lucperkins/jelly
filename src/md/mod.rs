mod code;
mod highlight;
mod parse;
mod title;

pub use parse::{ast, render};
pub use title::get_document_title;
