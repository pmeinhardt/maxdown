use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use clap::Parser;

mod io;
mod markdown;
mod template;

// The default HTML template embedded in the binary as a string
const DEFAULT_TEMPLATE: &str = include_str!("../templates/default-template.html");

/// Convert Markdown to HTML
#[derive(Parser, Debug)]
#[command(arg_required_else_help(true))]
#[command(version)]
struct Args {
    /// Path to the input Markdown file or "-" for stdin
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

/// Main function to handle command-line arguments and orchestrate the Markdown-to-HTML conversion
fn main() -> Result<()> {
    // Parse command-line arguments
    let args = Args::parse();

    // Read the Markdown input from a file or stdin
    let input = io::read(&args.path)
        .with_context(|| format!("Failed to read input from {:?}", args.path))?;

    // Convert the Markdown input to HTML
    let html = markdown::convert(&input, args.dangerous).map_err(|m| anyhow!(m))?;

    // Prepare values for template rendering
    let values: HashMap<String, String> = HashMap::from([
        ("base".to_string(), args.base.unwrap_or_default()),
        ("title".to_string(), args.title),
        ("content".to_string(), html.trim().to_string()),
    ]);

    // Read the custom template if provided, or use the default template
    let template = match args.template {
        Some(path) => {
            io::read(&path).with_context(|| format!("Failed to read template from {:?}", path))?
        }
        None => DEFAULT_TEMPLATE.to_string(),
    };

    // Render the final HTML by replacing placeholders in the template
    let result = template::render(&template, &values);

    // Write the output to a file or stdout
    // Use match expression to handle the output path
    match args.output {
        Some(path) => {
            io::write(&path, &result)
                .with_context(|| format!("Failed to write output to {:?}", path))?;
        }
        None => {
            println!("{}", result);
        }
    }

    Ok(())
}
