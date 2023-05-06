use maxdown::*;

use std::collections::HashMap;
use std::fmt::Display;
use std::fs::{self, read_to_string as read, write};
use std::io::prelude::*;
use std::io::{self, Error};
use std::process;

use clap::Parser;

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

trait UnwrapOrBail<T, E: Display> {
    fn unwrap_or_bail(self, message: &str) -> T;
}

impl<T, E: Display> UnwrapOrBail<T, E> for Result<T, E> {
    fn unwrap_or_bail(self, message: &str) -> T {
        self.unwrap_or_else(|error| bail(message, &error))
    }
}

fn bail<E: Display>(message: &str, error: &E) -> ! {
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

fn main() {
    let args = Args::parse();

    let input = slurp(&args.path).unwrap_or_bail("Failed to read input");
    let html = convert(&input, args.dangerous).unwrap();
    let base = args.base.unwrap_or(String::from(""));

    let values = HashMap::from([
        ("base", &*base),
        ("css", &*CSS),
        ("result", &*html),
        ("title", &*args.title),
    ]);

    let template = match args.template {
        Some(path) => read(&path).unwrap_or_bail("Failed to read template"),
        None => String::from(TEMPLATE),
    };

    let result = render(&template, &values);

    match args.output {
        Some(path) => write(path, result).unwrap_or_bail("Failed to write output"),
        None => println!("{}", result),
    }
}
