// Example: Data Types
fn main() {
    // Integer types
    let a: i8 = 127;
    let b: u8 = 255;
    let c: i16 = 32767;
    let d: u16 = 65535;
    let e: i32 = 2_147_483_647;
    let f: u32 = 4_294_967_295;
    
    println!("Integer examples: {} {} {} {} {} {}", a, b, c, d, e, f);
    
    // Integer literals
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';  // u8 only
    
    println!("Integer literals: {} {} {} {} {}", decimal, hex, octal, binary, byte);
    
    // Floating-point types
    let x = 2.0;      // f64 (default)
    let y: f32 = 3.0; // f32
    
    println!("Floating-point examples: {} {}", x, y);
    
    // Numeric operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    
    println!("Operations: {} {} {} {} {}", sum, difference, product, quotient, remainder);
    
    // Boolean type
    let t = true;
    let f: bool = false;
    
    println!("Boolean examples: {} {}", t, f);
    
    // Character type
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
    
    println!("Character examples: {} {} {}", c, z, heart_eyed_cat);
    
    // Tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;  // Destructuring
    
    println!("Tuple values: {} {} {}", x, y, z);
    println!("Tuple access: {} {} {}", tup.0, tup.1, tup.2);
    
    // Array type
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let repeated = [3; 5];  // [3, 3, 3, 3, 3]
    
    println!("Array first element: {}", arr[0]);
    println!("Array length: {}", arr.len());
    println!("Repeated array: {:?}", repeated);
    
    // String types
    let string_literal = "Hello";  // &str
    let string = String::from("Hello, world!");  // String
    
    println!("String examples: {} {}", string_literal, string);
}