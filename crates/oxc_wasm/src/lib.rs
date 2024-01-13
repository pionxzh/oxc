mod options;

use std::path::PathBuf;

use oxc::{
    allocator::Allocator,
    parser::Parser,
    span::SourceType,
};
use oxc_prettier::{Prettier, PrettierOptions};
use wasm_bindgen::prelude::*;

use crate::options::OxcParserOptions;

#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
#[derive(Default)]
pub struct Oxc {
    source_text: String,
    prettier_formatted_text: String,
}

#[wasm_bindgen]
impl Oxc {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { ..Self::default() }
    }

    #[wasm_bindgen(getter = sourceText)]
    pub fn source_text(&self) -> String {
        self.source_text.clone()
    }

    #[wasm_bindgen(setter = sourceText)]
    pub fn set_source_text(&mut self, source_text: String) {
        self.source_text = source_text;
    }

    #[wasm_bindgen(getter = prettierFormattedText)]
    pub fn prettier_formatted_text(&self) -> String {
        self.prettier_formatted_text.clone()
    }

    /// # Errors
    /// Serde serialization error
    #[wasm_bindgen]
    pub fn run(
        &mut self,
        parser_options: &OxcParserOptions,
    ) -> Result<(), serde_wasm_bindgen::Error> {
        let allocator = Allocator::default();
        let source_text = &self.source_text;
        let path = PathBuf::from("test.tsx");
        let source_type = SourceType::from_path(&path).unwrap_or_default();

        let ret = Parser::new(&allocator, source_text, source_type)
            .allow_return_outside_function(parser_options.allow_return_outside_function)
            .preserve_parens(false)
            .parse();
        let printed =
            Prettier::new(&allocator, source_text, ret.trivias, PrettierOptions::default())
                .build(&ret.program);
        self.prettier_formatted_text = printed;

        Ok(())
    }
}
