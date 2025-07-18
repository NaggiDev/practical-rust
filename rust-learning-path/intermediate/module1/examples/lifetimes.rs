// Example demonstrating Rust's lifetime system
fn main() {
    // Basic lifetime example
    let string1 = String::from("long string is long");
    let string2 = String::from("short");
    
    let result = longest(&string1, &string2);
    println!("The longest string is: {}", result);
    
    // Lifetime in structs
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let excerpt = Excerpt {
        part: first_sentence,
    };
    
    println!("Excerpt: {}", excerpt.part);
    
    // Lifetime elision
    let s = String::from("Hello, world!");
    let first = first_word(&s);
    println!("First word: {}", first);
    
    // Static lifetime
    let s: &'static str = "I have a static lifetime.";
    println!("Static string: {}", s);
}

// Function with explicit lifetime annotation
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Struct with lifetime annotation
struct Excerpt<'a> {
    part: &'a str,
}

// Function with elided lifetime
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

// Function with multiple lifetime parameters
fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

use std::fmt::Display;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_longest() {
        let string1 = String::from("long");
        let string2 = String::from("longer");
        
        let result = longest(&string1, &string2);
        assert_eq!(result, "longer");
    }
    
    #[test]
    fn test_first_word() {
        let s = String::from("hello world");
        
        let word = first_word(&s);
        assert_eq!(word, "hello");
    }
    
    #[test]
    fn test_excerpt() {
        let novel = String::from("First sentence. Second sentence.");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        
        let excerpt = Excerpt {
            part: first_sentence,
        };
        
        assert_eq!(excerpt.part, "First sentence");
    }
    
    #[test]
    fn test_longest_with_announcement() {
        let string1 = String::from("long");
        let string2 = String::from("longer");
        
        let result = longest_with_announcement(&string1, &string2, "Testing lifetimes");
        assert_eq!(result, "longer");
    }
}