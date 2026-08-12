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

    markdown::to_html_with_options(input, options).map(|html| transform_alerts(&html))
}

/// The recognized GitHub alert types.
const ALERT_TYPES: &[&str] = &["NOTE", "TIP", "IMPORTANT", "WARNING", "CAUTION"];

/// Post-processes HTML output to transform GitHub-style alert blockquotes into
/// `<div class="markdown-alert ...">` elements.
///
/// GitHub supports highlighted block-quotes using `[!TYPE]` as the first line:
///
/// ```markdown
/// > [!NOTE]
/// > Content here.
/// ```
///
/// The `markdown` crate renders these as plain blockquotes. This function
/// detects the pattern and transforms them into the expected alert HTML so that
/// the GitHub Markdown CSS classes (already embedded in the preview template)
/// render them correctly.
fn transform_alerts(html: &str) -> String {
    const BQ_OPEN: &str = "<blockquote>\n<p>[!";
    const BQ_CLOSE: &str = "</blockquote>";

    let mut result = String::with_capacity(html.len());
    let mut pos = 0;

    while pos < html.len() {
        let rest = &html[pos..];

        let Some(bq_offset) = rest.find(BQ_OPEN) else {
            result.push_str(rest);
            break;
        };

        // Absolute position of "[!" in the original string.
        let type_start = pos + bq_offset + BQ_OPEN.len();

        // Find the closing bracket that ends the alert type token.
        let Some(bracket_end) = html[type_start..].find(']') else {
            result.push_str(&html[pos..type_start]);
            pos = type_start;
            continue;
        };

        let type_upper = html[type_start..type_start + bracket_end].to_ascii_uppercase();

        if !ALERT_TYPES.contains(&type_upper.as_str()) {
            // Not a recognized alert type — copy prefix and keep scanning.
            result.push_str(&html[pos..type_start]);
            pos = type_start;
            continue;
        }

        // Absolute position right after `]`.
        let after_bracket_pos = type_start + bracket_end + 1;

        // Find the end of the enclosing blockquote.
        let Some(bq_end) = html[after_bracket_pos..].find(BQ_CLOSE) else {
            result.push_str(&html[pos..type_start]);
            pos = type_start;
            continue;
        };

        // `inner` is the HTML between `]` and `</blockquote>`.
        let inner = &html[after_bracket_pos..after_bracket_pos + bq_end];

        let css_class = type_upper.to_lowercase();
        let title = title_case(&type_upper);
        let content = extract_alert_content(inner);

        // Emit everything before this blockquote, then the alert div.
        result.push_str(&html[pos..pos + bq_offset]);
        result.push_str(&format!(
            "<div class=\"markdown-alert markdown-alert-{css_class}\">\n\
             <p class=\"markdown-alert-title\">{title}</p>\n\
             {content}</div>"
        ));

        pos = after_bracket_pos + bq_end + BQ_CLOSE.len();
    }

    result
}

/// Extracts the alert body from the raw HTML that follows the `]` token and
/// precedes `</blockquote>`.
///
/// The `markdown` crate can produce two shapes depending on whether the `[!TYPE]`
/// tag shares a paragraph with the first content line:
///
/// - `"\ncontent</p>\n..."` — type and first content line in the same `<p>`
/// - `"</p>\ncontent..."` — type was its own `<p>`, content follows separately
fn extract_alert_content(inner: &str) -> String {
    if let Some(rest) = inner.strip_prefix('\n') {
        // The type token was followed by more text in the same paragraph.
        if let Some(p_end) = rest.find("</p>") {
            let first = &rest[..p_end];
            let after = rest[p_end + 4..].trim_start_matches('\n');
            if first.is_empty() {
                // The type token shared its `<p>` with only a newline (e.g. `[!NOTE]\n`
                // followed by a blank continuation line). The empty paragraph is skipped
                // and subsequent paragraphs become the alert body.
                after.to_string()
            } else {
                format!("<p>{first}</p>\n{after}")
            }
        } else {
            rest.to_string()
        }
    } else if let Some(rest) = inner.strip_prefix("</p>\n") {
        // The type token was the sole content of its paragraph.
        rest.to_string()
    } else if let Some(rest) = inner.strip_prefix("</p>") {
        rest.to_string()
    } else {
        inner.to_string()
    }
}

/// Converts an ASCII-uppercase string to title case (e.g. `"NOTE"` → `"Note"`).
///
/// The input is assumed to be already uppercase ASCII, so only the characters
/// after the first need to be lowercased.
fn title_case(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_string() + &chars.as_str().to_lowercase(),
    }
}
