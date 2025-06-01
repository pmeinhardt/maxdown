use std::collections::HashMap;
use std::fs::{self, write};
use std::io;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use markdown;
use markdown::message::Message;

mod template;
use template::Template;

// The default HTML template embedded in the binary as a string
const DEFAULT_TEMPLATE: &str = include_str!("../templates/default-template.html");

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
    let values: HashMap<String, String> = HashMap::from([
        ("base".to_string(), args.base.unwrap_or_default()),
        ("title".to_string(), args.title),
        ("content".to_string(), html.trim().to_string()),
    ]);

    // Read the custom template if provided, or use the default template
    let template: Template = match args.template {
        Some(path) => read(&path)
            .with_context(|| format!("Failed to read template from {:?}", path))?
            .into(),
        None => DEFAULT_TEMPLATE.into(),
    };

    // Render the final HTML by replacing placeholders in the template
    let result = template.render(&values);

    // Write the output to a file or stdout
    // Use match expression to handle the output path
    match args.output {
        Some(path) => {
            write(&path, result).with_context(|| format!("Failed to write output to {:?}", path))?
        }
        None => {
            println!("{}", result)
        }
    }

    Ok(())
}
