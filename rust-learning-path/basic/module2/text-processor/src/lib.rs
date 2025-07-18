use std::fmt;
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

pub mod transformations;

/// Custom error type for text processor operations
#[derive(Debug)]
pub enum TextProcessorError {
    IoError(io::Error),
    InvalidArguments(String),
    InvalidOperation(String),
}

impl fmt::Display for TextProcessorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TextProcessorError::IoError(err) => write!(f, "I/O error: {}", err),
            TextProcessorError::InvalidArguments(msg) => write!(f, "Invalid arguments: {}", msg),
            TextProcessorError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
        }
    }
}

impl From<io::Error> for TextProcessorError {
    fn from(error: io::Error) -> Self {
        TextProcessorError::IoError(error)
    }
}

/// Read text from a file or stdin
///
/// # Arguments
/// * `source` - The source to read from ("-" for stdin, otherwise a file path)
///
/// # Returns
/// * `Result<String, TextProcessorError>` - The text content or an error
pub fn read_text(source: &str) -> Result<String, TextProcessorError> {
    if source == "-" {
        // Read from stdin
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        Ok(buffer)
    } else {
        // Read from file
        match fs::read_to_string(source) {
            Ok(content) => Ok(content),
            Err(e) => Err(TextProcessorError::IoError(e)),
        }
    }
}

/// Write text to a file or stdout
///
/// # Arguments
/// * `destination` - The destination to write to ("-" for stdout, otherwise a file path)
/// * `content` - The text content to write
///
/// # Returns
/// * `Result<(), TextProcessorError>` - Success or an error
pub fn write_text(destination: &str, content: &str) -> Result<(), TextProcessorError> {
    if destination == "-" {
        // Write to stdout
        io::stdout().write_all(content.as_bytes())?;
        Ok(())
    } else {
        // Write to file
        match fs::write(destination, content) {
            Ok(_) => Ok(()),
            Err(e) => Err(TextProcessorError::IoError(e)),
        }
    }
}