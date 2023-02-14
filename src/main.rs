use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use std::io::{self, Error};
use std::process;

use clap::Parser;
use markdown;
use regex::Regex;

const TEMPLATE: &str = include_str!("default-template.html");
const CSS: &str = include_str!("github.css");

/// Convert Markdown to HTML
#[derive(Parser, Debug)]
#[command(arg_required_else_help(true))]
#[command(version)]
struct Args {
    /// Path to the input markdown file or "-" for stdin
    path: String,

    /// Only use this if you trust the authors of the document
    #[arg(short, long)]
    dangerous: bool,

    /// Template to use for output
    #[arg(short, long, value_name = "path")]
    template: Option<String>,

    /// Title to pass to the template
    #[arg(long, value_name = "title", default_value = "Preview")]
    title: String,
}

fn bail(message: &str, error: &Error) -> ! {
    eprintln!("{message}: {error}");

    let _ = io::stdout().lock().flush();
    let _ = io::stderr().lock().flush();

    process::exit(1)
}

fn slurp(path: &str) -> Result<String, Error> {
    if path == "-" {
        return io::read_to_string(io::stdin());
    }

    fs::read_to_string(path)
}

fn read(path: &str) -> Result<String, Error> {
    fs::read_to_string(path)
}

fn convert(input: &str, dangerous: bool) -> Result<String, String> {
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

fn main() {
    let args = Args::parse();

    let input = slurp(&args.path).unwrap_or_else(|error| bail("Failed to read input", &error));
    let html = convert(&input, args.dangerous).unwrap();

    let values = HashMap::from([("css", &*CSS), ("result", &*html), ("title", &*args.title)]);

    let template = match args.template {
        Some(path) => read(&path).unwrap_or_else(|error| bail("Failed to read template", &error)),
        None => String::from(TEMPLATE),
    };

    println!("{}", render(&template, &values));
}
