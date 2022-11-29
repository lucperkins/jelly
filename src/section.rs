use crate::config::Config;
use crate::error::ContentError;
use crate::get_pages_in_dir;
use crate::page::Page;
use serde::Deserialize;
use std::fs::read_to_string;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Deserialize)]
struct SectionConfig {
    title: String,
}

pub struct Section {
    pub title: String,
    pub pages: Vec<Page>,
}

pub fn get_sections(config: &Config) -> Result<Vec<Section>, ContentError> {
    let mut sections: Vec<Section> = Vec::new();

    for entry in WalkDir::new(&config.root) {
        let maybe_entry = entry?;
        let path = maybe_entry.path();

        if path.is_dir() {
            let yaml_path = Path::new(path).join("_meta.yaml");
            if yaml_path.exists() {
                let yaml_file_str = read_to_string(yaml_path)?;
                let section_config: SectionConfig = serde_yaml::from_str(&yaml_file_str)?;
                let pages = get_pages_in_dir(path, config)?;
                let section = Section {
                    title: section_config.title,
                    pages,
                };
                sections.push(section);
            }
        }
    }

    Ok(sections)
}
