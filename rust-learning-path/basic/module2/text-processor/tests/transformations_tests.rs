use text_processor::transformations::{
    to_uppercase, to_lowercase, count_statistics, 
    replace_pattern, remove_duplicates, sort_lines
};

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for to_uppercase function
    #[test]
    fn test_to_uppercase() {
        let input = "Hello, World!";
        let result = to_uppercase(input);
        assert_eq!(result, "HELLO, WORLD!");
    }

    #[test]
    fn test_to_uppercase_with_mixed_case() {
        let input = "Hello, WORLD!";
        let result = to_uppercase(input);
        assert_eq!(result, "HELLO, WORLD!");
    }

    // Tests for to_lowercase function
    #[test]
    fn test_to_lowercase() {
        let input = "Hello, World!";
        let result = to_lowercase(input);
        assert_eq!(result, "hello, world!");
    }

    #[test]
    fn test_to_lowercase_with_mixed_case() {
        let input = "Hello, WORLD!";
        let result = to_lowercase(input);
        assert_eq!(result, "hello, world!");
    }

    // Tests for count_statistics function
    #[test]
    fn test_count_statistics_empty_string() {
        let input = "";
        let stats = count_statistics(input);
        assert_eq!(stats.char_count, 0);
        assert_eq!(stats.word_count, 0);
        assert_eq!(stats.line_count, 0);
    }

    #[test]
    fn test_count_statistics_single_line() {
        let input = "Hello, World!";
        let stats = count_statistics(input);
        assert_eq!(stats.char_count, 13);
        assert_eq!(stats.word_count, 2);
        assert_eq!(stats.line_count, 1);
    }

    #[test]
    fn test_count_statistics_multiple_lines() {
        let input = "Hello, World!\nThis is a test.\nMultiple lines.";
        let stats = count_statistics(input);
        assert_eq!(stats.char_count, 44);
        assert_eq!(stats.word_count, 8);
        assert_eq!(stats.line_count, 3);
    }

    // Tests for replace_pattern function
    #[test]
    fn test_replace_pattern() {
        let input = "Hello, World!";
        let result = replace_pattern(input, "World", "Rust");
        assert_eq!(result, "Hello, Rust!");
    }

    #[test]
    fn test_replace_pattern_multiple_occurrences() {
        let input = "test test test";
        let result = replace_pattern(input, "test", "rust");
        assert_eq!(result, "rust rust rust");
    }

    #[test]
    fn test_replace_pattern_no_match() {
        let input = "Hello, World!";
        let result = replace_pattern(input, "Rust", "Programming");
        assert_eq!(result, "Hello, World!");
    }

    // Tests for remove_duplicates function
    #[test]
    fn test_remove_duplicates_no_duplicates() {
        let input = "Line 1\nLine 2\nLine 3";
        let result = remove_duplicates(input);
        assert_eq!(result, "Line 1\nLine 2\nLine 3");
    }

    #[test]
    fn test_remove_duplicates_with_duplicates() {
        let input = "Line 1\nLine 2\nLine 1\nLine 3\nLine 2";
        let result = remove_duplicates(input);
        assert_eq!(result, "Line 1\nLine 2\nLine 3");
    }

    #[test]
    fn test_remove_duplicates_empty_string() {
        let input = "";
        let result = remove_duplicates(input);
        assert_eq!(result, "");
    }

    // Tests for sort_lines function
    #[test]
    fn test_sort_lines() {
        let input = "Line B\nLine C\nLine A";
        let result = sort_lines(input);
        assert_eq!(result, "Line A\nLine B\nLine C");
    }

    #[test]
    fn test_sort_lines_with_duplicates() {
        let input = "Line B\nLine A\nLine B\nLine C";
        let result = sort_lines(input);
        assert_eq!(result, "Line A\nLine B\nLine B\nLine C");
    }

    #[test]
    fn test_sort_lines_empty_string() {
        let input = "";
        let result = sort_lines(input);
        assert_eq!(result, "");
    }
}