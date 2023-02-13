use std::io;
use std::io::prelude::*;
use markdown as md;

fn convert(input: &str) -> Result<String, String> {
    let options = &md::Options {
        compile: md::CompileOptions {
            allow_dangerous_html: true,
            allow_dangerous_protocol: true,
            ..md::CompileOptions::gfm()
        },
        ..md::Options::gfm()
    };

    md::to_html_with_options(input, &options)
}

fn main() -> io::Result<()> {
    let mut stdin = io::stdin();
    let mut buffer = String::new();

    stdin.read_to_string(&mut buffer)?;

    match convert(&buffer) {
        Ok(html) => println!("{}", html),
        Err(msg) => println!("{}", msg),
    }

    Ok(())
}
