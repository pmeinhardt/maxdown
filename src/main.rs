use std::io;
use std::io::prelude::*;

use clap::Parser;
use markdown as md;

/// Convert Markdown to HTML
#[derive(Parser, Debug)]
struct Args {
    /// Only use this if you trust the authors of the document
    #[arg(short, long)]
    dangerous: bool,
}

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

fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut stdin = io::stdin();
    let mut buffer = String::new();

    stdin.read_to_string(&mut buffer)?;

    match convert(&buffer, args.dangerous) {
        Ok(html) => println!("{}", html),
        Err(msg) => println!("{}", msg),
    }

    Ok(())
}
