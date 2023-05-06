use std::collections::HashMap;

use markdown;
use regex::Regex;
use wasm_bindgen::prelude::*;

pub const TEMPLATE: &str = include_str!("default-template.html");
pub const CSS: &str = include_str!("github.css");

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

pub fn render(template: &str, values: &HashMap<&str, &str>) -> String {
    let mut result = String::from(template);

    for (key, value) in values {
        result = replace(&result, key, value);
    }

    result
}

fn replace(template: &str, key: &str, value: &str) -> String {
    let pattern = [r"\{\{\s*", key, r"\s*\}\}"].join("");
    let re = Regex::new(&pattern).unwrap();
    re.replace_all(template, value).to_string()
}

#[wasm_bindgen]
pub fn md(input: &str, dangerous: bool) -> Result<String, String> {
    let html = convert(input, dangerous)?;

    let values = HashMap::from([
        ("base", ""),
        ("css", &*CSS),
        ("result", &*html),
        ("title", "Maxdown Preview"),
    ]);

    let result = render(TEMPLATE, &values);

    Ok(result)
}
