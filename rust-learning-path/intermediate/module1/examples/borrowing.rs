// Example demonstrating Rust's borrowing rules
fn main() {
    // Basic borrowing example
    let mut value = 10;
    
    // Immutable borrows
    let ref1 = &value;
    let ref2 = &value;
    println!("References: {} {}", ref1, ref2);
    
    // After the immutable references are no longer used,
    // we can create a mutable reference
    let ref_mut = &mut value;
    *ref_mut += 10;
    println!("After mutation: {}", value);
    
    // Non-Lexical Lifetimes example
    let mut v = vec![1, 2, 3];
    
    // Create an immutable reference
    let first = &v[0];
    println!("First element: {}", first);
    // first is no longer used after this point
    
    // We can now mutate v, even though first is still in scope
    v.push(4);
    println!("Vector after push: {:?}", v);
    
    // Nested borrows example
    let mut outer = String::from("outer");
    
    {
        let inner = &mut outer;
        inner.push_str(" modified");
        println!("Inner reference: {}", inner);
        // inner goes out of scope here
    }
    
    // Now we can borrow outer again
    println!("Outer after modification: {}", outer);
}

// Function that takes a reference
fn print_length(s: &String) {
    println!("String length: {}", s.len());
}

// Function that takes a mutable reference
fn append_text(s: &mut String, text: &str) {
    s.push_str(text);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_borrowing() {
        let mut s = String::from("hello");
        
        // Test immutable borrowing
        let len = s.len();
        assert_eq!(len, 5);
        
        // Test mutable borrowing
        append_text(&mut s, " world");
        assert_eq!(s, "hello world");
    }
    
    #[test]
    fn test_multiple_immutable_borrows() {
        let s = String::from("hello");
        
        let r1 = &s;
        let r2 = &s;
        
        assert_eq!(r1, r2);
    }
    
    #[test]
    fn test_non_lexical_lifetimes() {
        let mut v = vec![1, 2, 3];
        
        // Use an immutable borrow
        let first = &v[0];
        assert_eq!(*first, 1);
        
        // After using first, we can mutate v
        v.push(4);
        assert_eq!(v, vec![1, 2, 3, 4]);
    }
}