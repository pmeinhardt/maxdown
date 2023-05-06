use markdown;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn convert(input: &str, dangerous: bool) -> Result<String, String> {
    let options = &markdown::Options {
        compile: markdown::CompileOptions {
            allow_dangerous_html: dangerous,
            allow_dangerous_protocol: dangerous,
            ..markdown::CompileOptions::gfm()
        },
        ..markdown::Options::gfm()
    };

    markdown::to_html_with_options(input, &options)
}
