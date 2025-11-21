use anyhow::Result;
use tree_sitter::{Language, Parser, Tree, ffi::TSLanguage};
use tree_sitter_tlaplus::LANGUAGE;

fn language() -> Language {
    let lang_fn = LANGUAGE.into_raw();
    let ptr = unsafe { lang_fn() } as *const TSLanguage;
    unsafe { Language::from_raw(ptr) }
}

pub struct TlaParser {
    parser: Parser,
}

impl TlaParser {
    pub fn new() -> Result<Self> {
        let mut parser = Parser::new();
        parser.set_language(&language())?;
        Ok(Self { parser })
    }

    pub fn parse(&mut self, source: &str) -> Option<Tree> {
        self.parser.parse(source, None)
    }
}
