use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Default, Clone, Copy)]
pub struct OxcRunOptions {
    syntax: bool,
    lint: bool,
    format: bool,
    prettier_format: bool,
    transform: bool,
    scope: bool,
    symbol: bool,
}

#[wasm_bindgen]
impl OxcRunOptions {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { syntax: true, lint: true, ..Self::default() }
    }

    #[wasm_bindgen(getter)]
    pub fn syntax(self) -> bool {
        self.format
    }

    #[wasm_bindgen(setter)]
    pub fn set_syntax(&mut self, yes: bool) {
        self.syntax = yes;
    }

    #[wasm_bindgen(getter)]
    pub fn lint(self) -> bool {
        self.lint
    }

    #[wasm_bindgen(setter)]
    pub fn set_lint(&mut self, yes: bool) {
        self.lint = yes;
    }

    #[wasm_bindgen(getter)]
    pub fn format(self) -> bool {
        self.format
    }

    #[wasm_bindgen(setter)]
    pub fn set_format(&mut self, yes: bool) {
        self.format = yes;
    }

    #[wasm_bindgen(getter)]
    pub fn prettier_format(self) -> bool {
        self.prettier_format
    }

    #[wasm_bindgen(setter)]
    pub fn set_prettier_format(&mut self, yes: bool) {
        self.prettier_format = yes;
    }

    #[wasm_bindgen(getter)]
    pub fn transform(self) -> bool {
        self.transform
    }

    #[wasm_bindgen(setter)]
    pub fn set_transform(&mut self, yes: bool) {
        self.transform = yes;
    }

    #[wasm_bindgen(getter)]
    pub fn scope(self) -> bool {
        self.scope
    }

    #[wasm_bindgen(setter)]
    pub fn set_scope(&mut self, yes: bool) {
        self.scope = yes;
    }

    #[wasm_bindgen(getter)]
    pub fn symbol(self) -> bool {
        self.symbol
    }

    #[wasm_bindgen(setter)]
    pub fn set_symbol(&mut self, yes: bool) {
        self.symbol = yes;
    }
}

#[wasm_bindgen]
#[derive(Default, Clone, Copy)]
pub struct OxcParserOptions {
    #[wasm_bindgen(js_name = allowReturnOutsideFunction)]
    pub allow_return_outside_function: bool,
}

#[wasm_bindgen]
impl OxcParserOptions {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }
}
