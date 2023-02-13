use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

use clap::Parser;
use markdown as md;
use regex::Regex;

/// Convert Markdown to HTML
#[derive(Parser, Debug)]
struct Args {
    /// Only use this if you trust the authors of the document
    #[arg(short, long)]
    dangerous: bool,

    /// Template to use for rendering
    #[arg(short, long, value_name = "path")]
    template: Option<String>,

    /// Title to pass to the template
    #[arg(long, value_name = "title", default_value = "Markdown")]
    title: String,
}

const TEMPLATE: &str = include_str!("default.html");

fn convert(input: &str, dangerous: bool) -> Result<String, String> {
    let options = &md::Options {
        compile: md::CompileOptions {
            allow_dangerous_html: dangerous,
            allow_dangerous_protocol: dangerous,
            ..md::CompileOptions::gfm()
        },
        ..md::Options::gfm()
    };

    md::to_html_with_options(input, &options)
}

fn replace(template: &str, key: &str, value: &str) -> String {
    let pattern = [r"\{\{\s*", key, r"\s*\}\}"].join("");
    let re = Regex::new(&pattern).unwrap();
    re.replace_all(template, value).to_string()
}

fn render(template: &str, values: &HashMap<&str, &str>) -> String {
    let mut result = String::from(template);

    for (key, value) in values {
        result = replace(&result, key, value);
    }

    result
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut stdin = io::stdin();
    let mut buffer = String::new();

    stdin.read_to_string(&mut buffer)?;

    let html = convert(&buffer, args.dangerous).unwrap();

    let values = HashMap::from([
        ("result", &*html),
        ("title", &*args.title),
    ]);

    println!("{}", render(TEMPLATE, &values));

    Ok(())
}
