use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use minijinja::{context, Environment};

mod markdown;

/// The default HTML template embedded into the binary as a string.
const DEFAULT_TEMPLATE: &str = include_str!("../templates/default-template.html");

/// Convert Markdown to HTML
#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// Path to the input Markdown file [default: stdin]
    path: Option<PathBuf>,

    /// Base URL to use for all relative URLs in the document
    #[arg(short, long, value_name = "url")]
    base: Option<String>,

    /// Only use this if you trust the authors of the document
    #[arg(long)]
    dangerous: bool,

    /// File to write output to [default: stdout]
    #[arg(short, long, value_name = "path")]
    output: Option<PathBuf>,

    /// Template to use for output [default: built-in template]
    #[arg(short, long, value_name = "path")]
    template: Option<PathBuf>,

    /// Title to pass to the template
    #[arg(long, value_name = "title", default_value = "Preview")]
    title: String,
}

/// Main function to handle command-line arguments and orchestrate the Markdown-to-HTML conversion
fn main() -> Result<()> {
    // Parse command-line arguments.
    let args = Args::parse();

    // Read the Markdown input from a file or from stdin.
    let source = match args.path {
        Some(ref path) => fs::read_to_string(&path)
            .with_context(|| format!("Failed to read input from {:?}", path))?,
        None => io::read_to_string(io::stdin()).context("Failed to read from stdin")?,
    };

    // Convert the Markdown input to HTML.
    let content = markdown::convert(&source, args.dangerous).map_err(|m| anyhow!(m))?;

    // Read the custom template, if provided, or use the default template.
    let template = match args.template {
        Some(ref path) => fs::read_to_string(&path)
            .with_context(|| format!("Failed to read template from {:?}", path))?,
        None => DEFAULT_TEMPLATE.to_string(),
    };

    // Generate the final HTML output by rendering the template.
    let mut env = Environment::empty();
    env.set_keep_trailing_newline(true);

    let ctx = context! {
        base => args.base.unwrap_or_default(),
        content => content.trim(),
        title => args.title,
    };

    let result = env
        .render_str(&template, ctx)
        .context("Failed to render template")?;

    // Direct output to a file, if a path was provided, or to stdout otherwise.
    let mut out: Box<dyn Write> = match args.output {
        Some(ref path) => {
            let file = File::create(&path)
                .with_context(|| format!("Failed to open output file {:?}", path))?;
            Box::new(BufWriter::new(file))
        }
        None => {
            let stream = io::stdout();
            Box::new(BufWriter::new(stream))
        }
    };

    // Write result to the output destination.
    write!(out, "{}", result).context("Failed to write output")?;
    out.flush().context("Failed to flush output")?;

    Ok(())
}
