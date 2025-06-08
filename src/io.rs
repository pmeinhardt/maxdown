use std::fs;
use std::io;
use std::path::Path;

/// Reads content from a file or stdin if the path is "-"
pub fn read(path: &Path) -> Result<String, io::Error> {
    if path == Path::new("-") {
        return io::read_to_string(io::stdin());
    }

    fs::read_to_string(path)
}

/// Writes content to a file
pub fn write(path: &Path, content: &str) -> Result<(), io::Error> {
    fs::write(path, content)
}
