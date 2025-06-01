use std::collections::HashMap;
use std::ops::Range;

// Define the delimiters for template placeholders
const OPEN: &str = "{{";
const CLOSE: &str = "}}";
const LEN: usize = 2;

/// Represents an instruction for template rendering
enum Instruction {
    Copy(Range<usize>),            // Copy a span of text from the source
    Replace(Range<usize>, String), // Replace a span of text with a value
}

/// Represents a simple string template with placeholders
pub struct Template {
    program: Vec<Instruction>,
    source: String,
}

impl Template {
    /// Creates a new template from a string
    pub fn new(source: String) -> Self {
        let len = source.len();

        let mut program = Vec::new();
        let mut offset = 0;

        while let Some(open) = source[offset..].find(OPEN) {
            let start = offset + open;

            if let Some(close) = source[start..].find(CLOSE) {
                let end = start + close + LEN;

                // Copy the text before the placeholder
                if start > offset {
                    program.push(Instruction::Copy(offset..start));
                }

                // Add a replace instruction for the placeholder
                let inner = &source[start + LEN..end - LEN];
                let key = inner.trim().to_string();
                program.push(Instruction::Replace(start..end, key));

                // Move the start position past the end of the placeholder
                offset = end;
            } else {
                // If no closing delimiter is found, treat the rest as regular text
                program.push(Instruction::Copy(offset..len));
                offset = len;
                break;
            }
        }

        if offset < len {
            // Copy any text remaining after the last placeholder
            program.push(Instruction::Copy(offset..len));
        }

        Template { program, source }
    }

    /// Renders the template with the provided values
    pub fn render(&self, values: &HashMap<String, String>) -> String {
        let mut result = String::new();

        for instruction in &self.program {
            match instruction {
                Instruction::Copy(range) => {
                    // Copy the text from the source within the specified range
                    result.push_str(&self.source[range.start..range.end]);
                }
                Instruction::Replace(range, key) => {
                    // Replace the placeholder with the corresponding value if it exists,
                    // otherwise keep the placeholder as is
                    match values.get(key) {
                        Some(value) => result.push_str(value),
                        None => result.push_str(&self.source[range.start..range.end]),
                    }
                }
            }
        }

        result
    }
}

impl From<String> for Template {
    fn from(source: String) -> Self {
        Template::new(source)
    }
}

impl From<&str> for Template {
    fn from(source: &str) -> Self {
        Template::new(source.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_rendering() {
        let template = Template::from("Hello, {{name}}! Welcome to {{place}}.");
        let values = HashMap::from([
            ("name".to_string(), "Alice".to_string()),
            ("place".to_string(), "Wonderland".to_string()),
        ]);

        let output = template.render(&values);

        assert_eq!(output, "Hello, Alice! Welcome to Wonderland.");
    }

    #[test]
    fn test_template_with_whitespace_placeholders() {
        let template = Template::from("Hello, {{ name }}! Welcome to {{ place }}.");
        let values = HashMap::from([
            ("name".to_string(), "Alice".to_string()),
            ("place".to_string(), "Wonderland".to_string()),
        ]);

        let output = template.render(&values);

        assert_eq!(output, "Hello, Alice! Welcome to Wonderland.");
    }

    #[test]
    fn test_template_rendering_with_missing_keys() {
        let template = Template::from("Hello, {{name}}! Welcome to {{place}}.");
        let values = HashMap::from([
            ("name".to_string(), "Alice".to_string()),
            // "place" is missing
        ]);

        let output = template.render(&values);

        assert_eq!(output, "Hello, Alice! Welcome to {{place}}.");
    }

    #[test]
    fn test_template_with_empty_placeholders() {
        let template = Template::from("Hello, {{}}! Welcome to {{place}}.");
        let values = HashMap::from([
            ("".to_string(), "Alice".to_string()),
            ("place".to_string(), "Wonderland".to_string()),
        ]);

        let output = template.render(&values);

        assert_eq!(output, "Hello, Alice! Welcome to Wonderland.");
    }

    #[test]
    fn test_template_with_unicode_characters() {
        let template = Template::from("Hello, {{ 🪪 }}! Welcome to {{ 📍 }}. 👋");
        let values = HashMap::from([
            ("🪪".to_string(), "👾".to_string()),
            ("📍".to_string(), "🌍".to_string()),
        ]);

        let output = template.render(&values);

        assert_eq!(output, "Hello, 👾! Welcome to 🌍. 👋");
    }

    #[test]
    fn test_template_without_placeholders() {
        let template = Template::from("Just a simple string.");
        let values = HashMap::from([]);

        let output = template.render(&values);

        assert_eq!(output, "Just a simple string.");
    }

    #[test]
    fn test_empty_template() {
        let template = Template::from("");
        let values = HashMap::from([]);

        let output = template.render(&values);

        assert_eq!(output, "");
    }
}
