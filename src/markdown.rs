use markdown;
use markdown::message::Message;

/// Converts Markdown input to HTML using the `markdown` crate
pub fn convert(input: &str, dangerous: bool) -> Result<String, Message> {
    let options = &markdown::Options {
        compile: markdown::CompileOptions {
            allow_dangerous_html: dangerous,     // allow potentially unsafe HTML
            allow_dangerous_protocol: dangerous, // allow unsafe protocols (e.g., `javascript:`)
            ..markdown::CompileOptions::gfm()    // use GitHub Flavored Markdown defaults
        },
        ..markdown::Options::gfm()
    };

    markdown::to_html_with_options(input, options)
}
