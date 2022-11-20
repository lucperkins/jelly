use gray_matter::engine::YAML;
use gray_matter::Matter;
use serde::Serialize;
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize)]
struct Page {
    //id: String,
    //path: String,
    //title: String,
    body: String,
}

#[derive(thiserror::Error, Debug)]
enum ContentError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

impl Page {
    fn from_file(mut file: File) -> Result<Self, ContentError> {
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents)?;

        let matter = Matter::<YAML>::new();
        let result = matter.parse(&contents);

        Ok(Page {
            body: result.content,
        })
    }
}

fn main() {
    let path = "./example/index.md";
    let file = File::open(path).unwrap();
    let page = Page::from_file(file).unwrap();
    println!("{}", page.body);
}
