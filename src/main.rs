use anyhow::{anyhow, Context, Result};
use markdown::message::Message;
use std::collections::HashMap;
use std::fs::{self, write};
use std::io;
use std::path::{Path, PathBuf};

use clap::Parser;
use markdown;

// The default HTML template embedded in the binary as a string
const TEMPLATE: &str = include_str!("default-template.html");

/// Convert Markdown to HTML
#[derive(Parser, Debug)]
#[command(arg_required_else_help(true))]
#[command(version)]
struct Args {
    /// Path to the input markdown file or "-" for stdin
    path: PathBuf,

    /// Base URL to use for all relative URLs in the document
    #[arg(short, long, value_name = "url")]
    base: Option<String>,

    /// Only use this if you trust the authors of the document
    #[arg(long)]
    dangerous: bool,

    /// File to write output to [default: stdout]
    #[arg(short, long, value_name = "path")]
    output: Option<PathBuf>,

    /// Template to use for output
    #[arg(short, long, value_name = "path")]
    template: Option<PathBuf>,

    /// Title to pass to the template
    #[arg(long, value_name = "title", default_value = "Preview")]
    title: String,
}

/// Reads content from a file or stdin if the path is "-"
fn read(path: &Path) -> Result<String, io::Error> {
    if path == Path::new("-") {
        return io::read_to_string(io::stdin());
    }

    fs::read_to_string(path)
}

/// Converts Markdown input to HTML using the `markdown` crate
fn convert(input: &str, dangerous: bool) -> Result<String, Message> {
    let options = &markdown::Options {
        compile: markdown::CompileOptions {
            allow_dangerous_html: dangerous,     // allow potentially unsafe HTML
            allow_dangerous_protocol: dangerous, // allow unsafe protocols (e.g., `javascript:`)
            ..markdown::CompileOptions::gfm()    // use GitHub Flavored Markdown defaults
        },
        ..markdown::Options::gfm()
    };

    markdown::to_html_with_options(input, &options)
}

/// Replaces placeholders in the template with the provided values
fn render(template: &str, values: &HashMap<&str, &str>) -> String {
    let mut result = String::new();
    let mut start = 0;

    while let Some(open) = template[start..].find("{{") {
        let open = start + open;
        if let Some(close) = template[open..].find("}}") {
            let close = open + close + 2;

            // Add everything before the placeholder
            result.push_str(&template[start..open]);

            // Extract the placeholder key and trim whitespace
            let key = &template[open + 2..close - 2].trim();
            if let Some(value) = values.get(key) {
                // Replace placeholder with value
                result.push_str(value);
            } else {
                // Keep the placeholder if no value is found
                result.push_str(&template[open..close]);
            }

            // Move the start pointer past the current placeholder
            start = close;
        } else {
            // No closing braces found; break the loop
            break;
        }
    }

    // Add the remaining part of the template
    result.push_str(&template[start..]);
    result
}

/// Main function to handle command-line arguments and orchestrate the Markdown-to-HTML conversion
fn main() -> Result<()> {
    // Parse command-line arguments
    let args = Args::parse();

    // Read the Markdown input from a file or stdin
    let input =
        read(&args.path).with_context(|| format!("Failed to read input from {:?}", args.path))?;

    // Convert the Markdown input to HTML
    let html = convert(&input, args.dangerous).map_err(|m| anyhow!(m))?;

    // Prepare values for template rendering
    let values = HashMap::from([
        ("base", args.base.as_deref().unwrap_or("")),
        ("title", args.title.as_ref()),
        ("content", html.trim()),
    ]);

    // Read the custom template if provided, or use the default template
    let template = match args.template {
        Some(path) => {
            read(&path).with_context(|| format!("Failed to read template from {:?}", path))?
        }
        None => TEMPLATE.to_string(),
    };

    // Render the final HTML by replacing placeholders in the template
    let result = render(&template, &values);

    // Write the output to a file or stdout
    match args.output {
        Some(path) => {
            write(&path, result).with_context(|| format!("Failed to write output to {:?}", path))?
        }
        None => println!("{}", result),
    }

    Ok(())
}
