//! # Compiler Plugin
//!
//! A Rust compiler plugin that provides custom linting functionality.
//! This plugin demonstrates how to work with Rust's compiler APIs to
//! create custom static analysis tools.
//!
//! ## Features
//!
//! - Custom lint detection for code patterns
//! - Rich diagnostic messages with suggestions
//! - Configurable lint levels
//! - Integration with standard Rust tooling

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Item, ItemFn};

mod diagnostics;
mod lint;

pub use diagnostics::*;
pub use lint::*;

/// Main entry point for the compiler plugin.
/// This procedural macro can be applied to functions to enable linting.
///
/// # Example
///
/// ```rust
/// use compiler_plugin::lint_function;
///
/// #[lint_function]
/// fn example_function() {
///     // Function body will be analyzed by the plugin
/// }
/// ```
#[proc_macro_attribute]
pub fn lint_function(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    
    // Analyze the function for potential issues
    let mut linter = FunctionLinter::new();
    let diagnostics = linter.analyze_function(&input);
    
    // Generate diagnostic messages if issues are found
    for diagnostic in diagnostics {
        diagnostic.emit();
    }
    
    // Return the original function unchanged
    let output = quote! {
        #input
    };
    
    output.into()
}

/// Derive macro that adds linting capabilities to structs.
/// This demonstrates how to create custom derive macros that work
/// with compiler analysis.
///
/// # Example
///
/// ```rust
/// use compiler_plugin::LintableStruct;
///
/// #[derive(LintableStruct)]
/// struct MyStruct {
///     field1: i32,
///     field2: String,
/// }
/// ```
#[proc_macro_derive(LintableStruct)]
pub fn derive_lintable_struct(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    // Analyze the struct definition
    let mut linter = StructLinter::new();
    let diagnostics = linter.analyze_struct(&input);
    
    // Emit diagnostics
    for diagnostic in diagnostics {
        diagnostic.emit();
    }
    
    // Generate implementation
    let name = &input.ident;
    let output = quote! {
        impl LintableStruct for #name {
            fn lint_info(&self) -> &'static str {
                "This struct has been analyzed by the compiler plugin"
            }
        }
    };
    
    output.into()
}

/// Trait that can be implemented by structs that support linting
pub trait LintableStruct {
    fn lint_info(&self) -> &'static str;
}

/// Procedural macro for analyzing entire modules
/// This demonstrates more advanced compiler plugin capabilities
#[proc_macro]
pub fn analyze_module(input: TokenStream) -> TokenStream {
    let items = syn::parse_macro_input!(input with syn::Block);
    
    let mut module_linter = ModuleLinter::new();
    
    // Analyze each item in the module
    for stmt in &items.stmts {
        if let syn::Stmt::Item(item) = stmt {
            let diagnostics = module_linter.analyze_item(item);
            for diagnostic in diagnostics {
                diagnostic.emit();
            }
        }
    }
    
    // Return the original module
    quote! {
        {
            #items
        }
    }.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use syn::parse_quote;

    #[test]
    fn test_function_linter_creation() {
        let linter = FunctionLinter::new();
        assert_eq!(linter.lint_count(), 0);
    }

    #[test]
    fn test_struct_linter_creation() {
        let linter = StructLinter::new();
        assert_eq!(linter.lint_count(), 0);
    }

    #[test]
    fn test_diagnostic_creation() {
        let diagnostic = Diagnostic::new(
            DiagnosticLevel::Warning,
            "Test warning".to_string(),
            None,
        );
        assert_eq!(diagnostic.level(), DiagnosticLevel::Warning);
        assert_eq!(diagnostic.message(), "Test warning");
    }
}