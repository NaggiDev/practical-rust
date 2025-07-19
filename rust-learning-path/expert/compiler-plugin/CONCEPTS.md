# Compiler Plugin Concepts

This document explains the key Rust concepts and compiler internals demonstrated in the Compiler Plugin project.

## Table of Contents

1. [Procedural Macros](#procedural-macros)
2. [Compiler Architecture](#compiler-architecture)
3. [Abstract Syntax Trees (AST)](#abstract-syntax-trees-ast)
4. [Token Streams](#token-streams)
5. [Diagnostic System](#diagnostic-system)
6. [Static Analysis](#static-analysis)
7. [Visitor Pattern](#visitor-pattern)
8. [Compiler APIs](#compiler-apis)

## Procedural Macros

Procedural macros are Rust's mechanism for metaprogramming - code that generates code. Unlike declarative macros (`macro_rules!`), procedural macros are functions that take token streams as input and produce token streams as output.

### Types of Procedural Macros

1. **Function-like macros** (`#[proc_macro]`)
2. **Derive macros** (`#[proc_macro_derive]`)
3. **Attribute macros** (`#[proc_macro_attribute]`)

```rust
// Function-like macro
#[proc_macro]
pub fn my_macro(input: TokenStream) -> TokenStream {
    // Process input and generate output
}

// Derive macro
#[proc_macro_derive(MyTrait)]
pub fn derive_my_trait(input: TokenStream) -> TokenStream {
    // Generate trait implementation
}

// Attribute macro
#[proc_macro_attribute]
pub fn my_attribute(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Process the attributed item
}
```

### Key Concepts

- **Token Streams**: The raw representation of Rust code as a sequence of tokens
- **Parsing**: Converting token streams into structured data (AST)
- **Code Generation**: Creating new token streams from structured data
- **Hygiene**: Ensuring generated code doesn't interfere with existing code

## Compiler Architecture

The Rust compiler follows a multi-stage pipeline:

```
Source Code → Lexing → Parsing → AST → HIR → MIR → LLVM IR → Machine Code
```

### Compilation Stages

1. **Lexing**: Converting source text into tokens
2. **Parsing**: Building an Abstract Syntax Tree (AST)
3. **AST → HIR**: Lowering to High-level Intermediate Representation
4. **HIR → MIR**: Lowering to Mid-level Intermediate Representation
5. **MIR → LLVM IR**: Code generation
6. **LLVM IR → Machine Code**: Final compilation

### Plugin Integration Points

Compiler plugins typically operate at the AST level, allowing them to:
- Analyze code structure
- Generate diagnostics
- Transform code (with limitations)
- Provide IDE integration

## Abstract Syntax Trees (AST)

An AST is a tree representation of the syntactic structure of source code. Each node represents a construct in the programming language.

### AST Node Types in Rust

```rust
// Examples of AST nodes from the syn crate
pub enum Item {
    Const(ItemConst),
    Enum(ItemEnum),
    Fn(ItemFn),
    Struct(ItemStruct),
    // ... many more
}

pub enum Expr {
    Array(ExprArray),
    Call(ExprCall),
    If(ExprIf),
    Match(ExprMatch),
    // ... many more
}
```

### Traversing ASTs

The visitor pattern is commonly used to traverse ASTs:

```rust
impl<'ast> Visit<'ast> for MyVisitor {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        // Process function
        visit::visit_item_fn(self, node); // Continue traversal
    }
    
    fn visit_expr(&mut self, node: &'ast Expr) {
        // Process expression
        visit::visit_expr(self, node); // Continue traversal
    }
}
```

## Token Streams

Token streams represent Rust code as a sequence of tokens. They're the interface between the compiler and procedural macros.

### Token Types

- **Identifiers**: Variable names, function names, etc.
- **Literals**: Numbers, strings, characters
- **Punctuation**: Operators, delimiters
- **Keywords**: `fn`, `struct`, `impl`, etc.

### Working with Token Streams

```rust
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn my_macro(input: TokenStream) -> TokenStream {
    // Parse the input
    let parsed = parse_macro_input!(input as syn::DeriveInput);
    
    // Generate output
    let output = quote! {
        // Generated code here
    };
    
    output.into()
}
```

## Diagnostic System

The diagnostic system provides feedback to developers about their code. It includes errors, warnings, and informational messages.

### Diagnostic Components

1. **Level**: Error, Warning, Info
2. **Message**: Description of the issue
3. **Span**: Location in source code
4. **Suggestions**: Proposed fixes
5. **Error Codes**: Unique identifiers for diagnostic types

### Creating Diagnostics

```rust
pub struct Diagnostic {
    level: DiagnosticLevel,
    message: String,
    suggestion: Option<String>,
    span: Option<Span>,
}

impl Diagnostic {
    pub fn emit(&self) {
        // Output diagnostic to compiler
        eprintln!("{}: {}", self.level, self.message);
        if let Some(suggestion) = &self.suggestion {
            eprintln!("  = help: {}", suggestion);
        }
    }
}
```

## Static Analysis

Static analysis examines code without executing it. Compiler plugins perform static analysis to detect issues and suggest improvements.

### Types of Analysis

1. **Syntactic Analysis**: Based on code structure
2. **Semantic Analysis**: Based on meaning and types
3. **Control Flow Analysis**: Based on execution paths
4. **Data Flow Analysis**: Based on data usage

### Analysis Techniques

- **Pattern Matching**: Detecting specific code patterns
- **Metrics Collection**: Measuring complexity, size, etc.
- **Rule Checking**: Enforcing coding standards
- **Dependency Analysis**: Understanding relationships between code elements

## Visitor Pattern

The visitor pattern separates algorithms from the data structures they operate on. It's essential for AST traversal.

### Implementation

```rust
pub trait Visit<'ast> {
    fn visit_item(&mut self, item: &'ast Item) {
        visit_item(self, item);
    }
    
    fn visit_expr(&mut self, expr: &'ast Expr) {
        visit_expr(self, expr);
    }
    
    // Many more visit methods...
}

// Default traversal functions
pub fn visit_item<'ast, V: Visit<'ast>>(visitor: &mut V, item: &'ast Item) {
    match item {
        Item::Fn(item_fn) => visitor.visit_item_fn(item_fn),
        Item::Struct(item_struct) => visitor.visit_item_struct(item_struct),
        // Handle all variants...
    }
}
```

### Custom Visitors

```rust
struct MyLinter {
    issues: Vec<String>,
}

impl<'ast> Visit<'ast> for MyLinter {
    fn visit_item_fn(&mut self, func: &'ast ItemFn) {
        // Check function-specific rules
        if func.sig.ident.to_string().len() > 30 {
            self.issues.push("Function name too long".to_string());
        }
        
        // Continue traversal
        visit::visit_item_fn(self, func);
    }
}
```

## Compiler APIs

Rust provides various APIs for interacting with the compiler, though many are unstable and require nightly Rust.

### Stable APIs

- **syn**: Parsing Rust syntax
- **quote**: Generating Rust code
- **proc-macro2**: Procedural macro utilities

### Unstable APIs (Nightly Only)

- **rustc_ast**: Compiler's AST representation
- **rustc_span**: Source location tracking
- **rustc_errors**: Diagnostic emission
- **rustc_hir**: High-level IR

### Using Stable APIs

```rust
use syn::{parse_quote, ItemFn};
use quote::quote;

// Parse Rust code
let func: ItemFn = parse_quote! {
    fn example() {
        println!("Hello, world!");
    }
};

// Generate Rust code
let generated = quote! {
    fn generated_function() {
        println!("Generated code!");
    }
};
```

## Advanced Concepts

### Span Information

Spans track the location of code elements in source files:

```rust
use proc_macro2::Span;

pub struct SpanInfo {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub length: usize,
}
```

### Hygiene

Procedural macros must be hygienic - they shouldn't accidentally capture or interfere with identifiers in the calling code:

```rust
// Good: Uses qualified paths
let output = quote! {
    std::println!("Safe generated code");
};

// Potentially problematic: Might conflict with user code
let output = quote! {
    println!("Might conflict");
};
```

### Error Handling in Macros

```rust
#[proc_macro_derive(MyTrait)]
pub fn derive_my_trait(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    match generate_impl(&input) {
        Ok(tokens) => tokens,
        Err(error) => error.to_compile_error(),
    }
}
```

## Best Practices

1. **Use Stable APIs**: Prefer `syn` and `quote` over unstable compiler APIs
2. **Provide Good Diagnostics**: Clear messages with actionable suggestions
3. **Handle Errors Gracefully**: Don't panic in procedural macros
4. **Test Thoroughly**: Use both unit tests and integration tests
5. **Document Well**: Explain what your plugin does and how to use it
6. **Consider Performance**: Large codebases will run your plugin frequently

## Integration with Development Tools

### IDE Support

Modern IDEs can integrate with compiler plugins to provide:
- Real-time diagnostics
- Code suggestions
- Refactoring support
- Syntax highlighting for generated code

### Continuous Integration

Compiler plugins can be integrated into CI pipelines to:
- Enforce coding standards
- Detect potential issues early
- Generate reports on code quality

### Custom Tooling

Plugins can be the foundation for custom development tools:
- Code formatters
- Documentation generators
- Metric collectors
- Refactoring tools

## Conclusion

Compiler plugins represent a powerful way to extend Rust's capabilities. By understanding the concepts covered in this document, you can create tools that help developers write better, more maintainable code. The key is to start simple, focus on providing value to developers, and gradually expand functionality as you become more comfortable with the compiler's internals.