use anyhow::{anyhow, Context, Result};
use std::collections::HashMap;
use std::fs::{self, read_to_string as read, write};
use std::io;

use clap::Parser;
use markdown;

const TEMPLATE: &str = include_str!("default-template.html");

/// Convert Markdown to HTML
#[derive(Parser, Debug)]
#[command(arg_required_else_help(true))]
#[command(version)]
struct Args {
    /// Path to the input markdown file or "-" for stdin
    path: String,

    /// Base URL to use for all relative URLs in the document
    #[arg(short, long, value_name = "url")]
    base: Option<String>,

    /// Only use this if you trust the authors of the document
    #[arg(long)]
    dangerous: bool,

    /// File to write output to [default: stdout]
    #[arg(short, long, value_name = "path")]
    output: Option<String>,

    /// Template to use for output
    #[arg(short, long, value_name = "path")]
    template: Option<String>,

    /// Title to pass to the template
    #[arg(long, value_name = "title", default_value = "Preview")]
    title: String,
}

fn slurp(path: &str) -> Result<String, io::Error> {
    if path == "-" {
        return io::read_to_string(io::stdin());
    }

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

fn render(template: &str, values: &HashMap<&str, &str>) -> String {
    let mut result = String::from(template);

    for (key, value) in values {
        let pattern = format!("{{{{ {key} }}}}");
        result = result.replace(&pattern, value)
    }

    result
}

fn main() -> Result<()> {
    let args = Args::parse();

    let input =
        slurp(&args.path).with_context(|| format!("Failed to read input from {}", args.path))?;

    let html = convert(&input, args.dangerous).map_err(|m| anyhow!(m))?;
    let base = args.base.unwrap_or(String::from(""));

    let values = HashMap::from([
        ("base", &*base),
        ("content", html.trim()),
        ("title", &*args.title),
    ]);

    let template = match args.template {
        Some(path) => {
            read(&path).with_context(|| format!("Failed to read template from {path}"))?
        }
        None => String::from(TEMPLATE),
    };

    let result = render(&template, &values);

    match args.output {
        Some(path) => {
            write(&path, result).with_context(|| format!("Failed to write output to {path}"))?
        }
        None => println!("{}", result),
    }

    Ok(())
}
