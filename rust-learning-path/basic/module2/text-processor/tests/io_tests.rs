use std::fs;
use std::io::Write;
use text_processor::{read_text, write_text, TextProcessorError};

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;
    use tempfile::tempdir;

    // Tests for read_text function
    #[test]
    fn test_read_text_from_file() {
        // Create a temporary directory
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        
        // Write test content to the file
        let test_content = "Hello, World!";
        fs::write(&file_path, test_content).unwrap();
        
        // Read the content using our function
        let result = read_text(file_path.to_str().unwrap());
        
        // Check the result
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), test_content);
    }

    #[test]
    fn test_read_text_file_not_found() {
        // Try to read a non-existent file
        let result = read_text("non_existent_file.txt");
        
        // Check that we get an error
        assert!(result.is_err());
        match result {
            Err(TextProcessorError::IoError(_)) => assert!(true),
            _ => assert!(false, "Expected IoError"),
        }
    }

    // Tests for write_text function
    #[test]
    fn test_write_text_to_file() {
        // Create a temporary directory
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("output.txt");
        
        // Write content using our function
        let test_content = "Hello, Rust!";
        let result = write_text(file_path.to_str().unwrap(), test_content);
        
        // Check the result
        assert!(result.is_ok());
        
        // Verify the file content
        let mut file = File::open(&file_path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        assert_eq!(content, test_content);
    }

    // Note: Testing stdin/stdout is more complex and typically requires
    // mocking or integration tests. These tests focus on file I/O.
}