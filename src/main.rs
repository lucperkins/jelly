use gray_matter::engine::YAML;
use gray_matter::{Matter, Pod};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Serialize)]
struct Page {
    //id: String,
    //path: String,
    title: String,
    body: String,
}

#[derive(Deserialize)]
struct FrontMatter {
    title: Option<String>,
}

impl Default for FrontMatter {
    fn default() -> Self {
        Self { title: None }
    }
}

#[derive(thiserror::Error, Debug)]
enum ContentError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("error parsing front matter")]
    FrontMatter,
}

impl Page {
    fn from_path(path: &Path) -> Result<Self, ContentError> {
        let mut file = File::open(path.as_os_str())?;
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents)?;

        let matter = Matter::<YAML>::new();
        let result = matter.parse(&contents);

        let front: FrontMatter = match result.data {
            Some(f) => f.deserialize()?,
            None => FrontMatter::default(),
        };

        let title: String = front.title.unwrap_or(String::from(path.to_string_lossy()));

        Ok(Page {
            body: result.content,
            title,
        })
    }
}

fn main() {
    let path = "./example/index.md";
    let file = File::open(path).unwrap();
    let page = Page::from_path(&Path::new(path)).unwrap();
    println!("(title: {}, body: {})", page.title, page.body);
}
