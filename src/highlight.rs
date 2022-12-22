use syntect::{
    html::{ClassStyle, ClassedHTMLGenerator},
    parsing::SyntaxSet,
    util::LinesWithEndings,
};

use crate::error::Error;

pub struct Highlighter {
    syntaxes: SyntaxSet,
}

impl Default for Highlighter {
    fn default() -> Self {
        Self {
            syntaxes: SyntaxSet::load_defaults_newlines(),
        }
    }
}

impl Highlighter {
    pub fn highlight(&self, language: &str, code: &str) -> Result<String, Error> {
        match self.syntaxes.find_syntax_by_token(language) {
            Some(sx) => {
                let mut html_generator = ClassedHTMLGenerator::new_with_class_style(
                    sx,
                    &self.syntaxes,
                    ClassStyle::Spaced,
                );
                for line in LinesWithEndings::from(code) {
                    html_generator.parse_html_for_line_which_includes_newline(line)?;
                }
                Ok(html_generator.finalize())
            }
            None => Err(Error::Highlight(format!(
                "no syntax found for language {}",
                language
            ))),
        }
    }
}
