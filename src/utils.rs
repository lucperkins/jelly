use crate::config::TitleConfig;
use crate::error::ContentError;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use titlecase::titlecase;

pub fn name_from_path(path: &Path, title_config: &TitleConfig) -> String {
    let stem = path.file_stem().unwrap();
    let stem_str = stem.to_string_lossy();

    #[allow(clippy::single_char_pattern)]
    let deslugged = stem_str.replace("-", " ");

    if title_config.title_case {
        titlecase(&deslugged)
    } else if title_config.first_letter_capitalized {
        capitalize_first_letter(&deslugged)
    } else {
        deslugged
    }
}

pub fn get_file(path: &Path) -> Result<String, ContentError> {
    let mut file = File::open(path.as_os_str())?;
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn capitalize_first_letter(s: &str) -> String {
    s[0..1].to_uppercase() + &s[1..]
}

// TODO: find a built-in function for this
pub fn get_or_none<T>(items: Vec<T>) -> Option<Vec<T>> {
    if items.is_empty() {
        None
    } else {
        Some(items)
    }
}
