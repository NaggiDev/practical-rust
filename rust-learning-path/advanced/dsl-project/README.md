# Domain-Specific Language (DSL) Project

## Learning Objectives

In this project, you'll learn to create a Domain-Specific Language using Rust's powerful macro system. You'll build a simple configuration DSL that demonstrates both declarative and procedural macros.

## What You'll Learn

- Declarative macros (`macro_rules!`)
- Procedural macros (derive, attribute, and function-like)
- Macro hygiene and scoping
- Token parsing and manipulation
- Code generation techniques

## Project Overview

You'll create a configuration DSL called `ConfigLang` that allows users to define application configurations in a more readable and type-safe way than traditional formats like JSON or YAML.

### Example DSL Usage

```rust
config! {
    app "MyApp" {
        version: "1.0.0",
        debug: true,
        
        database {
            host: "localhost",
            port: 5432,
            name: "myapp_db"
        },
        
        server {
            host: "0.0.0.0",
            port: 8080,
            workers: 4
        }
    }
}
```

This DSL will generate type-safe Rust structs and provide validation at compile time.

## Prerequisites

- Understanding of Rust structs and enums
- Basic knowledge of traits and generics
- Familiarity with pattern matching
- Understanding of Rust's ownership system

## Step-by-Step Implementation

### Step 1: Project Setup and Basic Declarative Macro

**Concepts Applied**: Basic macro syntax, token matching, code generation

1. Set up the project structure with `Cargo.toml`
2. Create a simple declarative macro for basic configuration
3. Implement token parsing for key-value pairs
4. Generate basic struct definitions

**Implementation Tasks**:
- Create `lib.rs` with basic macro infrastructure
- Implement `simple_config!` macro for flat configurations
- Add basic token matching patterns
- Generate struct with fields

### Step 2: Nested Configuration Support

**Concepts Applied**: Recursive macro patterns, nested token trees

1. Extend the macro to handle nested configurations
2. Implement recursive parsing for nested blocks
3. Generate nested struct definitions
4. Handle different data types (strings, numbers, booleans)

**Implementation Tasks**:
- Extend macro to parse nested `{}` blocks
- Implement recursive token tree processing
- Generate nested struct definitions
- Add type inference for literal values

### Step 3: Procedural Macro Foundation

**Concepts Applied**: Procedural macros, `syn` and `quote` crates, token streams

1. Set up procedural macro infrastructure
2. Create a derive macro for configuration validation
3. Implement basic token stream parsing
4. Generate validation methods

**Implementation Tasks**:
- Add `syn`, `quote`, and `proc-macro2` dependencies
- Create `derive(ConfigValidate)` macro
- Parse struct definitions
- Generate validation trait implementations

### Step 4: Advanced DSL Features

**Concepts Applied**: Attribute macros, complex code generation, error handling

1. Implement attribute macros for field validation
2. Add default value support
3. Create environment variable integration
4. Implement configuration merging

**Implementation Tasks**:
- Create `#[config_field]` attribute macro
- Add validation rules (required, range, pattern)
- Implement environment variable substitution
- Create configuration merging functionality

### Step 5: Error Handling and Diagnostics

**Concepts Applied**: Compile-time error reporting, span information, diagnostic messages

1. Implement comprehensive error handling
2. Add helpful compile-time error messages
3. Create span-aware diagnostics
4. Add macro debugging utilities

**Implementation Tasks**:
- Implement custom error types for macro expansion
- Add span information to error messages
- Create helpful diagnostic messages
- Add macro expansion debugging tools

### Step 6: Testing and Documentation

**Concepts Applied**: Macro testing strategies, documentation generation

1. Create comprehensive test suite for macros
2. Test both successful and error cases
3. Add documentation examples
4. Create usage examples

**Implementation Tasks**:
- Write unit tests for macro expansion
- Test error conditions and messages
- Add integration tests with real configurations
- Create comprehensive documentation

## Extension Challenges

1. **Performance Optimization**: Implement compile-time optimization for generated code
2. **IDE Integration**: Add support for syntax highlighting and completion
3. **Serialization Support**: Generate serialization/deserialization code
4. **Configuration Validation**: Add runtime validation with custom rules
5. **Multiple Output Formats**: Support generating code for different targets

## Key Concepts Covered

### Declarative Macros
- `macro_rules!` syntax and patterns
- Token matching and repetition
- Hygiene and variable capture
- Recursive macro patterns

### Procedural Macros
- Function-like procedural macros
- Derive macros for automatic trait implementation
- Attribute macros for code annotation
- Token stream manipulation

### Advanced Macro Techniques
- Error handling in macros
- Span information and diagnostics
- Code generation patterns
- Macro composition and reuse

## Testing Your Implementation

Run the tests to verify your implementation:

```bash
cargo test
```

Test the DSL with example configurations:

```bash
cargo run --example basic_config
cargo run --example nested_config
cargo run --example validation_example
```

## Resources

- [The Rust Programming Language - Macros](https://doc.rust-lang.org/book/ch19-06-macros.html)
- [The Little Book of Rust Macros](https://veykril.github.io/tlborm/)
- [syn crate documentation](https://docs.rs/syn/)
- [quote crate documentation](https://docs.rs/quote/)
- [proc-macro2 crate documentation](https://docs.rs/proc-macro2/)

## Next Steps

After completing this project, you'll have a solid understanding of Rust's macro system. Consider exploring:
- More complex DSL designs
- Integration with existing configuration systems
- Performance optimization techniques
- Contributing to macro-based crates in the ecosystem