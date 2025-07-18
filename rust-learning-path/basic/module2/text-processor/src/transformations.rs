use std::collections::HashSet;

/// Statistics about a text
pub struct TextStatistics {
    pub char_count: usize,
    pub word_count: usize,
    pub line_count: usize,
}

/// Convert text to uppercase
///
/// # Arguments
/// * `text` - The input text
///
/// # Returns
/// * `String` - The uppercase version of the input text
pub fn to_uppercase(text: &str) -> String {
    text.to_uppercase()
}

/// Convert text to lowercase
///
/// # Arguments
/// * `text` - The input text
///
/// # Returns
/// * `String` - The lowercase version of the input text
pub fn to_lowercase(text: &str) -> String {
    text.to_lowercase()
}

/// Count statistics (characters, words, lines) in text
///
/// # Arguments
/// * `text` - The input text
///
/// # Returns
/// * `TextStatistics` - Statistics about the text
pub fn count_statistics(text: &str) -> TextStatistics {
    let char_count = text.chars().count();
    let word_count = text.split_whitespace().count();
    let line_count = text.lines().count();
    
    TextStatistics {
        char_count,
        word_count,
        line_count,
    }
}

/// Replace all occurrences of a pattern with replacement text
///
/// # Arguments
/// * `text` - The input text
/// * `pattern` - The pattern to search for
/// * `replacement` - The text to replace the pattern with
///
/// # Returns
/// * `String` - The text with replacements applied
pub fn replace_pattern(text: &str, pattern: &str, replacement: &str) -> String {
    text.replace(pattern, replacement)
}

/// Remove duplicate lines from text
///
/// # Arguments
/// * `text` - The input text
///
/// # Returns
/// * `String` - The text with duplicate lines removed
pub fn remove_duplicates(text: &str) -> String {
    let mut seen = HashSet::new();
    let mut result = String::new();
    
    for line in text.lines() {
        if seen.insert(line) {
            result.push_str(line);
            result.push('\n');
        }
    }
    
    // Remove trailing newline if the original text didn't have one
    if !text.ends_with('\n') && result.ends_with('\n') {
        result.pop();
    }
    
    result
}

/// Sort lines alphabetically
///
/// # Arguments
/// * `text` - The input text
///
/// # Returns
/// * `String` - The text with lines sorted alphabetically
pub fn sort_lines(text: &str) -> String {
    let mut lines: Vec<&str> = text.lines().collect();
    lines.sort();
    
    let mut result = lines.join("\n");
    
    // Add trailing newline if the original text had one
    if text.ends_with('\n') {
        result.push('\n');
    }
    
    result
}