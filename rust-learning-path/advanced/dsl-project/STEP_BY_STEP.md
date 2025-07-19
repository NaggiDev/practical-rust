# Step-by-Step Implementation Guide: Domain-Specific Language Project

This guide walks you through implementing a Domain-Specific Language (DSL) using Rust's macro system. You'll build a configuration DSL that demonstrates both declarative and procedural macros.

## Overview

You'll create a configuration DSL called `ConfigLang` that allows users to define application configurations in a more readable and type-safe way than traditional formats. The project demonstrates:

- Declarative macros (`macro_rules!`)
- Procedural macros (derive, attribute, and function-like)
- Token parsing and manipulation
- Code generation techniques
- Macro hygiene and error handling

## Step 1: Project Setup and Basic Declarative Macro

### Learning Objectives
- Understand basic macro syntax and token matching
- Learn how to generate code from macro patterns
- Implement simple token parsing for key-value pairs

### Concepts Applied
- Basic `macro_rules!` syntax
- Token matching patterns (`$name:type`)
- Code generation with `$()*` repetition
- Struct generation and implementation blocks

### Implementation Tasks

#### 1.1 Set up the project structure

First, ensure your `Cargo.toml` is configured for procedural macros:

```toml
[package]
name = "dsl-project"
version = "0.1.0"
edition = "2021"

[lib]
proc-macro = true

[dependencies]
syn = { version = "2.0", features = ["full", "extra-traits"] }
quote = "1.0"
proc-macro2 = "1.0"

[dev-dependencies]
trybuild = "1.0"
```

#### 1.2 Create the basic declarative macro

In `src/lib.rs`, implement the `simple_config!` macro:

```rust
/// A simple declarative macro for basic configuration
#[macro_export]
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
        
        impl Default for $struct_name {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}
```

**Key Points:**
- `$struct_name:ident` captures the struct name as an identifier
- `$($field:ident: $value:expr),*` captures field-value pairs with repetition
- `$(,)?` optionally matches a trailing comma
- The expansion generates a struct with public fields and constructor

#### 1.3 Test the basic macro

Create a test to verify the macro works:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_config_macro() {
        simple_config! {
            TestConfig {
                name: "test",
                version: "1.0.0"
            }
        }
        
        let config = TestConfig::new();
        assert_eq!(config.name, "test");
        assert_eq!(config.version, "1.0.0");
    }
}
```

#### 1.4 Run and verify

```bash
cargo test test_simple_config_macro
```

### What You've Learned

- How to define declarative macros with `macro_rules!`
- Token matching patterns and repetition syntax
- Basic code generation for structs and implementations
- How macros expand at compile time

## Step 2: Nested Configuration Support

### Learning Objectives
- Implement recursive macro patterns
- Handle nested token trees
- Generate complex struct hierarchies

### Concepts Applied
- Recursive macro calls
- Internal macro rules (using `@` prefix)
- Token tree (`tt`) matching
- Accumulator patterns in macros

### Implementation Tasks

#### 2.1 Extend the macro for nested configurations

Add the main `config!` macro that supports nested blocks:

```rust
#[macro_export]
macro_rules! config {
    // Entry point: app name with configuration block
    (app $app_name:literal { $($content:tt)* }) => {
        config!(@parse_config AppConfig, $($content)*);
    };
    
    // Internal rule: parse configuration content
    (@parse_config $struct_name:ident, $($content:tt)*) => {
        config!(@build_struct $struct_name, [], $($content)*);
    };
    
    // Handle simple field: value pairs
    (@build_struct $struct_name:ident, [$($fields:tt)*], $field:ident: $value:expr, $($rest:tt)*) => {
        config!(@build_struct $struct_name, [$($fields)* ($field, $value, simple),], $($rest)*);
    };
    
    // Handle nested configuration blocks
    (@build_struct $struct_name:ident, [$($fields:tt)*], $field:ident { $($nested:tt)* }, $($rest:tt)*) => {
        config!(@create_nested_struct $field, $($nested)*);
        config!(@build_struct $struct_name, [$($fields)* ($field, $field, nested),], $($rest)*);
    };
    
    // Base case: generate final struct
    (@build_struct $struct_name:ident, [$($fields:tt)*],) => {
        config!(@generate_struct $struct_name, $($fields)*);
    };
    
    // Generate nested struct recursively
    (@create_nested_struct $nested_name:ident, $($content:tt)*) => {
        config!(@build_struct $nested_name, [], $($content)*);
    };
    
    // Generate struct with mixed field types
    (@generate_struct $struct_name:ident, $(($field:ident, $value:expr, $kind:ident),)*) => {
        #[derive(Debug, Clone)]
        pub struct $struct_name {
            $(pub $field: config!(@field_type $value, $kind),)*
        }
        
        impl $struct_name {
            pub fn new() -> Self {
                Self {
                    $(
                        $field: config!(@field_default $value, $kind),
                    )*
                }
            }
        }
        
        impl Default for $struct_name {
            fn default() -> Self {
                Self::new()
            }
        }
    };
    
    // Helper: determine field type
    (@field_type $value:expr, simple) => { &'static str };
    (@field_type $value:ident, nested) => { $value };
    
    // Helper: determine field default value
    (@field_default $value:expr, simple) => { $value };
    (@field_default $value:ident, nested) => { $value::new() };
}
```

#### 2.2 Test nested configuration

```rust
#[test]
fn test_nested_config() {
    config! {
        app "TestApp" {
            version: "1.0.0",
            debug: "true",
            
            database {
                host: "localhost",
                port: "5432"
            }
        }
    }
    
    let config = AppConfig::new();
    assert_eq!(config.version, "1.0.0");
    assert_eq!(config.database.host, "localhost");
}
```

### What You've Learned

- Recursive macro patterns and internal rules
- Token tree matching with `tt`
- Accumulator patterns for building complex data
- How to generate nested struct hierarchies

## Step 3: Procedural Macro Foundation

### Learning Objectives
- Set up procedural macro infrastructure
- Parse token streams with `syn`
- Generate code with `quote`
- Create derive macros

### Concepts Applied
- Procedural macro attributes
- Token stream parsing
- AST (Abstract Syntax Tree) manipulation
- Trait implementation generation

### Implementation Tasks

#### 3.1 Create the validation trait

```rust
/// Trait for configuration validation
pub trait ConfigValidate {
    fn validate(&self) -> Result<(), String>;
    fn is_valid(&self) -> bool;
}

/// Basic validation trait for primitive types
pub trait Validate {
    fn is_valid(&self) -> bool;
}

impl Validate for String {
    fn is_valid(&self) -> bool {
        !self.is_empty()
    }
}

impl Validate for &str {
    fn is_valid(&self) -> bool {
        !self.is_empty()
    }
}

impl Validate for u16 {
    fn is_valid(&self) -> bool {
        *self > 0
    }
}

impl Validate for u32 {
    fn is_valid(&self) -> bool {
        *self > 0
    }
}

impl Validate for bool {
    fn is_valid(&self) -> bool {
        true // booleans are always valid
    }
}
```

#### 3.2 Implement the derive macro

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(ConfigValidate)]
pub fn derive_config_validate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    let validation_methods = match &input.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(fields) => {
                    let field_validations = fields.named.iter().map(|field| {
                        let field_name = &field.ident;
                        
                        quote! {
                            if !self.#field_name.is_valid() {
                                return Err(format!("Invalid value for field '{}'", stringify!(#field_name)));
                            }
                        }
                    });
                    
                    quote! {
                        #(#field_validations)*
                    }
                }
                _ => quote! {},
            }
        }
        _ => quote! {},
    };
    
    let expanded = quote! {
        impl ConfigValidate for #name {
            fn validate(&self) -> Result<(), String> {
                #validation_methods
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

#### 3.3 Test the derive macro

```rust
#[test]
fn test_config_validation_derive() {
    #[derive(ConfigValidate)]
    struct TestConfig {
        name: String,
        port: u16,
        enabled: bool,
    }
    
    let valid_config = TestConfig {
        name: "test".to_string(),
        port: 8080,
        enabled: true,
    };
    
    let invalid_config = TestConfig {
        name: "".to_string(), // Invalid: empty string
        port: 0,              // Invalid: zero port
        enabled: false,
    };
    
    assert!(valid_config.is_valid());
    assert!(!invalid_config.is_valid());
}
```

### What You've Learned

- How to create procedural macros with `proc_macro`
- Parsing Rust syntax with `syn`
- Generating code with `quote`
- Creating derive macros for automatic trait implementation

## Step 4: Advanced DSL Features

### Learning Objectives
- Implement attribute macros
- Add complex validation rules
- Handle environment variable integration
- Create function-like procedural macros

### Concepts Applied
- Attribute macro parsing
- Complex code generation
- Environment variable handling
- Advanced token stream manipulation

### Implementation Tasks

#### 4.1 Create attribute macro for field validation

```rust
#[proc_macro_attribute]
pub fn config_field(args: TokenStream, input: TokenStream) -> TokenStream {
    // For now, this is a placeholder that parses but doesn't modify
    // In a full implementation, this would parse validation rules
    let _args = args; // Parse validation rules from args
    input // Return the field unchanged for now
}
```

#### 4.2 Implement function-like procedural macro

```rust
#[proc_macro]
pub fn advanced_config(input: TokenStream) -> TokenStream {
    // This demonstrates a function-like procedural macro
    let _input_str = input.to_string();
    
    // Generate a configuration struct with environment variable support
    let expanded = quote! {
        #[derive(Debug, Clone)]
        pub struct AdvancedConfig {
            pub name: String,
            pub port: u16,
            pub debug: bool,
        }
        
        impl AdvancedConfig {
            pub fn new() -> Self {
                Self {
                    name: std::env::var("APP_NAME").unwrap_or_else(|_| "DefaultApp".to_string()),
                    port: std::env::var("APP_PORT")
                        .ok()
                        .and_then(|p| p.parse().ok())
                        .unwrap_or(8080),
                    debug: cfg!(debug_assertions),
                }
            }
        }
        
        impl Default for AdvancedConfig {
            fn default() -> Self {
                Self::new()
            }
        }
    };
    
    TokenStream::from(expanded)
}
```

#### 4.3 Test advanced features

```rust
#[test]
fn test_advanced_config_macro() {
    advanced_config! {
        struct TestAdvancedConfig {
            name: String = "TestApp",
            port: u16 = 3000,
            debug: bool = true,
        }
    }
    
    let config = AdvancedConfig::new();
    assert!(!config.name.is_empty());
    assert!(config.port > 0);
}
```

### What You've Learned

- Creating attribute macros for code annotation
- Function-like procedural macros
- Environment variable integration
- Complex code generation patterns

## Step 5: Error Handling and Diagnostics

### Learning Objectives
- Implement comprehensive error handling
- Add helpful compile-time error messages
- Create span-aware diagnostics
- Add debugging utilities

### Concepts Applied
- Compile-time error reporting
- Span information for precise error locations
- Custom error types for macro expansion
- Diagnostic message generation

### Implementation Tasks

#### 5.1 Add error handling to procedural macros

```rust
use syn::{Error, spanned::Spanned};

fn validate_struct_for_config(input: &DeriveInput) -> Result<(), Error> {
    match &input.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(_) => Ok(()),
                Fields::Unnamed(_) => Err(Error::new_spanned(
                    input,
                    "ConfigValidate can only be derived for structs with named fields"
                )),
                Fields::Unit => Err(Error::new_spanned(
                    input,
                    "ConfigValidate cannot be derived for unit structs"
                )),
            }
        }
        Data::Enum(_) => Err(Error::new_spanned(
            input,
            "ConfigValidate can only be derived for structs, not enums"
        )),
        Data::Union(_) => Err(Error::new_spanned(
            input,
            "ConfigValidate can only be derived for structs, not unions"
        )),
    }
}
```

#### 5.2 Update the derive macro with error handling

```rust
#[proc_macro_derive(ConfigValidate)]
pub fn derive_config_validate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    // Validate input before processing
    if let Err(error) = validate_struct_for_config(&input) {
        return error.to_compile_error().into();
    }
    
    let name = &input.ident;
    
    // Rest of implementation...
    let expanded = quote! {
        impl ConfigValidate for #name {
            fn validate(&self) -> Result<(), String> {
                // Validation logic
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

#### 5.3 Test error conditions

```rust
#[test]
fn test_validation_errors() {
    #[derive(ConfigValidate)]
    struct ErrorTestConfig {
        name: String,
        port: u16,
    }
    
    let config = ErrorTestConfig {
        name: "".to_string(),
        port: 0,
    };
    
    match config.validate() {
        Ok(_) => panic!("Expected validation to fail"),
        Err(msg) => {
            assert!(msg.contains("Invalid value"));
        }
    }
}
```

### What You've Learned

- Error handling in procedural macros
- Using spans for precise error locations
- Creating helpful diagnostic messages
- Testing error conditions in macros

## Step 6: Testing and Documentation

### Learning Objectives
- Create comprehensive test suites for macros
- Test both successful and error cases
- Add documentation examples
- Use `trybuild` for compilation testing

### Concepts Applied
- Macro testing strategies
- Integration testing
- Documentation testing
- Compilation failure testing

### Implementation Tasks

#### 6.1 Create comprehensive integration tests

Update `tests/integration_tests.rs`:

```rust
use dsl_project::*;

#[test]
fn test_simple_config_generation() {
    simple_config! {
        SimpleTestConfig {
            app_name: "TestApp",
            version: "2.0.0",
            debug: "true"
        }
    }
    
    let config = SimpleTestConfig::new();
    assert_eq!(config.app_name, "TestApp");
    assert_eq!(config.version, "2.0.0");
    assert_eq!(config.debug, "true");
}

#[test]
fn test_macro_hygiene() {
    // Test that macros don't interfere with each other
    simple_config! {
        Config1 {
            name: "config1"
        }
    }
    
    simple_config! {
        Config2 {
            name: "config2"
        }
    }
    
    let c1 = Config1::new();
    let c2 = Config2::new();
    
    assert_eq!(c1.name, "config1");
    assert_eq!(c2.name, "config2");
}
```

#### 6.2 Add documentation tests

Add doc tests to your macros:

```rust
/// A simple declarative macro for basic configuration
/// 
/// # Example
/// 
/// ```
/// use dsl_project::simple_config;
/// 
/// simple_config! {
///     AppConfig {
///         name: "MyApp",
///         version: "1.0.0"
///     }
/// }
/// 
/// let config = AppConfig::new();
/// assert_eq!(config.name, "MyApp");
/// ```
#[macro_export]
macro_rules! simple_config {
    // Implementation...
}
```

#### 6.3 Run all tests

```bash
# Run unit tests
cargo test

# Run integration tests
cargo test --test integration_tests

# Run doc tests
cargo test --doc

# Run examples
cargo run --example basic_config
cargo run --example nested_config
cargo run --example validation_example
```

### What You've Learned

- Comprehensive testing strategies for macros
- Integration testing with real usage scenarios
- Documentation testing for examples
- How to verify macro behavior across different use cases

## Extension Challenges

Once you've completed the basic implementation, try these advanced challenges:

### 1. Performance Optimization
- Implement compile-time optimization for generated code
- Minimize the size of generated structs
- Optimize validation logic

### 2. IDE Integration
- Add support for syntax highlighting
- Implement code completion hints
- Create better error messages with suggestions

### 3. Serialization Support
- Generate `serde` serialization/deserialization code
- Support multiple serialization formats
- Add custom serialization logic

### 4. Configuration Validation
- Add runtime validation with custom rules
- Implement range checking for numeric values
- Add pattern matching for string fields

### 5. Multiple Output Formats
- Support generating code for different targets
- Create bindings for other languages
- Generate documentation from configuration schemas

## Key Takeaways

By completing this project, you've learned:

1. **Declarative Macros**: Pattern matching and code generation
2. **Procedural Macros**: Token stream manipulation and AST parsing
3. **Error Handling**: Compile-time diagnostics and span information
4. **Testing**: Comprehensive testing strategies for macros
5. **DSL Design**: Creating user-friendly domain-specific languages

These skills are fundamental for creating powerful metaprogramming tools in Rust and understanding how many popular crates implement their functionality.

## Next Steps

- Explore existing macro-based crates like `serde`, `clap`, or `diesel`
- Contribute to open-source projects that use macros
- Create your own DSL for a specific problem domain
- Study advanced macro techniques in the Rust ecosystem