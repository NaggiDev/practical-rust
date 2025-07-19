# Rust Macro Concepts - DSL Project

This document explains the key Rust macro concepts demonstrated in the DSL project, providing detailed explanations of how macros work and how they're applied in creating domain-specific languages.

## Table of Contents

1. [Introduction to Macros](#introduction-to-macros)
2. [Declarative Macros](#declarative-macros)
3. [Procedural Macros](#procedural-macros)
4. [Token Streams and Parsing](#token-streams-and-parsing)
5. [Macro Hygiene](#macro-hygiene)
6. [Error Handling in Macros](#error-handling-in-macros)
7. [Advanced Techniques](#advanced-techniques)

## Introduction to Macros

Macros in Rust are a form of metaprogramming that allows you to write code that writes other code. They operate at compile time, transforming source code before it's compiled into machine code.

### Why Use Macros?

- **Code Generation**: Automatically generate repetitive code
- **Domain-Specific Languages**: Create custom syntax for specific problem domains
- **Compile-Time Computation**: Perform calculations and validations at compile time
- **Zero-Cost Abstractions**: Generate efficient code without runtime overhead

### Types of Macros

Rust has two main types of macros:

1. **Declarative Macros** (`macro_rules!`): Pattern-based macros that match and replace code
2. **Procedural Macros**: Function-like macros that operate on token streams

## Declarative Macros

Declarative macros use pattern matching to transform code. They're defined with `macro_rules!` and use a syntax similar to `match` expressions.

### Basic Syntax

```rust
macro_rules! macro_name {
    (pattern) => {
        expansion
    };
}
```

### Pattern Matching

Declarative macros use special syntax for pattern matching:

- `$name:type` - Captures a token of the specified type
- `$(...)*` - Matches zero or more repetitions
- `$(...)+` - Matches one or more repetitions
- `$(...)?` - Matches zero or one occurrence

### Token Types

Common token types used in patterns:

- `ident` - Identifiers (variable names, function names, etc.)
- `expr` - Expressions
- `ty` - Types
- `pat` - Patterns
- `stmt` - Statements
- `block` - Code blocks
- `item` - Items (functions, structs, etc.)
- `literal` - Literal values
- `tt` - Token trees (any valid token sequence)

### Example: Simple Configuration Macro

```rust
macro_rules! simple_config {
    ($struct_name:ident { $($field:ident: $value:expr),* $(,)? }) => {
        #[derive(Debug, Clone)]
        pub struct $struct_name {
            $(pub $field: &'static str,)*
        }
        
        impl $struct_name {
            pub fn new() -> Self {
                Self {
                    $($field: $value,)*
                }
            }
        }
    };
}
```

**Explanation**:
- `$struct_name:ident` captures the struct name
- `$($field:ident: $value:expr),*` captures field-value pairs with repetition
- `$(,)?` optionally matches a trailing comma
- The expansion generates a struct with the captured fields and a constructor

### Recursive Macros

Macros can call themselves recursively to handle complex nested structures:

```rust
macro_rules! config {
    // Base case: generate final struct
    (@generate_struct $name:ident, [$($fields:tt)*]) => {
        // Generate struct definition
    };
    
    // Recursive case: process more fields
    (@parse_fields $name:ident, [$($acc:tt)*], $field:ident: $value:expr, $($rest:tt)*) => {
        config!(@parse_fields $name, [$($acc)* ($field, $value),], $($rest)*);
    };
    
    // Entry point
    ($name:ident { $($content:tt)* }) => {
        config!(@parse_fields $name, [], $($content)*);
    };
}
```

**Key Concepts**:
- **Internal Rules**: Rules starting with `@` are internal helpers
- **Accumulator Pattern**: Building up results in recursive calls
- **Token Tree Matching**: Using `tt` to match arbitrary token sequences

## Procedural Macros

Procedural macros are more powerful and flexible than declarative macros. They're written as Rust functions that take token streams as input and produce token streams as output.

### Types of Procedural Macros

1. **Function-like macros**: `my_macro!(tokens)`
2. **Derive macros**: `#[derive(MyTrait)]`
3. **Attribute macros**: `#[my_attribute]`

### Setting Up Procedural Macros

To create procedural macros, you need:

1. A crate with `proc-macro = true` in `Cargo.toml`
2. Dependencies on `syn`, `quote`, and `proc-macro2`
3. Functions marked with the appropriate procedural macro attributes

```toml
[lib]
proc-macro = true

[dependencies]
syn = { version = "2.0", features = ["full"] }
quote = "1.0"
proc-macro2 = "1.0"
```

### Derive Macros

Derive macros automatically implement traits for structs and enums:

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ConfigValidate)]
pub fn derive_config_validate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    let expanded = quote! {
        impl ConfigValidate for #name {
            fn validate(&self) -> Result<(), String> {
                // Validation logic here
                Ok(())
            }
            
            fn is_valid(&self) -> bool {
                self.validate().is_ok()
            }
        }
    };
    
    TokenStream::from(expanded)
}
```

**Key Components**:
- `parse_macro_input!`: Parses the input token stream
- `quote!`: Generates code using template syntax
- `#name`: Interpolates variables into the generated code

### Attribute Macros

Attribute macros can modify the items they're applied to:

```rust
#[proc_macro_attribute]
pub fn config_field(args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the attribute arguments
    let args = parse_macro_input!(args as AttributeArgs);
    
    // Parse the item being annotated
    let input = parse_macro_input!(input as ItemStruct);
    
    // Modify the item based on the arguments
    let expanded = quote! {
        // Modified item
    };
    
    TokenStream::from(expanded)
}
```

### Function-like Macros

Function-like procedural macros provide the most flexibility:

```rust
#[proc_macro]
pub fn advanced_config(input: TokenStream) -> TokenStream {
    // Parse the input however you need
    let config = parse_config(input);
    
    // Generate code based on the parsed input
    let expanded = generate_config_code(config);
    
    TokenStream::from(expanded)
}
```

## Token Streams and Parsing

Understanding token streams is crucial for working with procedural macros.

### Token Streams

A `TokenStream` is a sequence of tokens that represents Rust code. Tokens include:

- **Identifiers**: Variable names, function names, etc.
- **Literals**: Numbers, strings, characters
- **Punctuation**: Operators, delimiters
- **Keywords**: `fn`, `struct`, `impl`, etc.

### Parsing with `syn`

The `syn` crate provides tools for parsing Rust syntax:

```rust
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(MyTrait)]
pub fn my_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    match &input.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(fields) => {
                    // Handle named fields
                    for field in &fields.named {
                        let field_name = &field.ident;
                        let field_type = &field.ty;
                        // Process field
                    }
                }
                Fields::Unnamed(fields) => {
                    // Handle tuple structs
                }
                Fields::Unit => {
                    // Handle unit structs
                }
            }
        }
        Data::Enum(data_enum) => {
            // Handle enums
        }
        Data::Union(data_union) => {
            // Handle unions
        }
    }
    
    // Generate code...
}
```

### Code Generation with `quote`

The `quote` crate provides a template system for generating code:

```rust
use quote::quote;

let field_name = &some_field.ident;
let field_type = &some_field.ty;

let generated = quote! {
    impl SomeTrait for SomeStruct {
        fn some_method(&self) -> #field_type {
            self.#field_name.clone()
        }
    }
};
```

**Template Features**:
- `#var`: Interpolate variables
- `#(#items)*`: Repeat for each item in an iterator
- `#(#items),*`: Repeat with comma separation

## Macro Hygiene

Macro hygiene prevents name collisions between macro-generated code and user code.

### Hygienic Macros

Rust macros are hygienic by default, meaning:

- Variables introduced in macros don't interfere with user variables
- Macros can't accidentally capture user variables
- Each macro expansion has its own scope

### Example of Hygiene

```rust
macro_rules! create_function {
    ($name:ident) => {
        fn $name() {
            let x = 42; // This x doesn't interfere with user's x
            println!("x = {}", x);
        }
    };
}

fn main() {
    let x = 10;
    create_function!(my_func);
    my_func(); // Prints "x = 42", not "x = 10"
    println!("x = {}", x); // Prints "x = 10"
}
```

### Breaking Hygiene

Sometimes you need to break hygiene intentionally:

```rust
macro_rules! declare_variable {
    ($name:ident, $value:expr) => {
        let $name = $value; // This creates a variable in the caller's scope
    };
}
```

## Error Handling in Macros

Proper error handling in macros improves the developer experience.

### Compile-Time Errors

Macros can generate compile-time errors:

```rust
macro_rules! ensure_positive {
    ($value:expr) => {
        if $value <= 0 {
            compile_error!("Value must be positive");
        }
    };
}
```

### Procedural Macro Errors

Procedural macros can return detailed error information:

```rust
use syn::Error;

#[proc_macro_derive(MyTrait)]
pub fn my_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    match validate_input(&input) {
        Ok(_) => {
            // Generate code
        }
        Err(error) => {
            return error.to_compile_error().into();
        }
    }
}

fn validate_input(input: &DeriveInput) -> Result<(), Error> {
    match &input.data {
        Data::Struct(_) => Ok(()),
        _ => Err(Error::new_spanned(
            input,
            "MyTrait can only be derived for structs"
        )),
    }
}
```

### Span Information

Spans provide location information for better error messages:

```rust
use syn::spanned::Spanned;

let error = Error::new(
    field.span(),
    "This field is not supported"
);
```

## Advanced Techniques

### Macro Composition

Macros can call other macros to build complex functionality:

```rust
macro_rules! config_with_validation {
    ($($content:tt)*) => {
        config! { $($content)* }
        
        impl ConfigValidate for AppConfig {
            fn validate(&self) -> Result<(), String> {
                // Validation logic
                Ok(())
            }
        }
    };
}
```

### Conditional Compilation

Macros can generate different code based on conditions:

```rust
macro_rules! debug_config {
    ($($content:tt)*) => {
        config! { $($content)* }
        
        #[cfg(debug_assertions)]
        impl AppConfig {
            pub fn debug_info(&self) {
                println!("Debug: {:?}", self);
            }
        }
    };
}
```

### Macro Testing

Testing macros requires special techniques:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_macro_expansion() {
        simple_config! {
            TestConfig {
                name: "test"
            }
        }
        
        let config = TestConfig::new();
        assert_eq!(config.name, "test");
    }
}
```

For procedural macros, use the `trybuild` crate to test compilation:

```rust
#[test]
fn test_derive_macro() {
    let t = trybuild::TestCases::new();
    t.pass("tests/pass/*.rs");
    t.compile_fail("tests/fail/*.rs");
}
```

## Best Practices

1. **Keep macros simple**: Complex logic should be in regular functions
2. **Provide good error messages**: Use spans and descriptive messages
3. **Document macro behavior**: Include examples and explain generated code
4. **Test thoroughly**: Test both success and failure cases
5. **Consider alternatives**: Sometimes traits or generics are better than macros
6. **Use hygiene appropriately**: Understand when to break hygiene
7. **Optimize for readability**: Generated code should be understandable

## Common Pitfalls

1. **Infinite recursion**: Be careful with recursive macro calls
2. **Token type mismatches**: Ensure patterns match the expected input
3. **Hygiene issues**: Understand variable scoping in macros
4. **Complex error messages**: Simplify error reporting for users
5. **Over-engineering**: Don't use macros when simpler solutions exist

## Conclusion

Macros are a powerful feature of Rust that enable metaprogramming and domain-specific languages. Understanding both declarative and procedural macros, along with concepts like token streams, hygiene, and error handling, allows you to create sophisticated code generation tools and DSLs that provide excellent developer experiences while maintaining Rust's safety guarantees.