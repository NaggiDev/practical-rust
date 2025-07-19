//! # Lint Implementation
//!
//! This module contains the core linting logic for the compiler plugin.
//! It demonstrates how to traverse and analyze Rust code using the syn crate.

use syn::{visit::Visit, DeriveInput, Expr, ItemFn, Local, Pat, Stmt, Item};
use crate::diagnostics::{Diagnostic, DiagnosticLevel};

/// Linter for analyzing functions
pub struct FunctionLinter {
    diagnostics: Vec<Diagnostic>,
    unused_variables: Vec<String>,
    complex_expressions: usize,
}

impl FunctionLinter {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
            unused_variables: Vec::new(),
            complex_expressions: 0,
        }
    }

    /// Analyze a function for potential issues
    pub fn analyze_function(&mut self, func: &ItemFn) -> Vec<Diagnostic> {
        self.diagnostics.clear();
        self.unused_variables.clear();
        self.complex_expressions = 0;

        // Visit the function to collect information
        self.visit_item_fn(func);

        // Apply linting rules
        self.check_function_complexity(func);
        self.check_naming_conventions(func);
        self.check_unused_variables();

        self.diagnostics.clone()
    }

    fn check_function_complexity(&mut self, func: &ItemFn) {
        if self.complex_expressions > 5 {
            let diagnostic = Diagnostic::new(
                DiagnosticLevel::Warning,
                format!(
                    "Function '{}' has high complexity ({} complex expressions). Consider breaking it down.",
                    func.sig.ident,
                    self.complex_expressions
                ),
                Some("Consider extracting some logic into separate functions".to_string()),
            );
            self.diagnostics.push(diagnostic);
        }
    }

    fn check_naming_conventions(&mut self, func: &ItemFn) {
        let name = func.sig.ident.to_string();
        
        // Check for snake_case convention
        if !name.chars().all(|c| c.is_lowercase() || c == '_' || c.is_numeric()) {
            let diagnostic = Diagnostic::new(
                DiagnosticLevel::Warning,
                format!("Function '{}' should use snake_case naming convention", name),
                Some(format!("Consider renaming to '{}'", to_snake_case(&name))),
            );
            self.diagnostics.push(diagnostic);
        }

        // Check for overly long names
        if name.len() > 30 {
            let diagnostic = Diagnostic::new(
                DiagnosticLevel::Info,
                format!("Function '{}' has a very long name ({} characters)", name, name.len()),
                Some("Consider using a shorter, more concise name".to_string()),
            );
            self.diagnostics.push(diagnostic);
        }
    }

    fn check_unused_variables(&mut self) {
        for var in &self.unused_variables {
            if !var.starts_with('_') {
                let diagnostic = Diagnostic::new(
                    DiagnosticLevel::Warning,
                    format!("Variable '{}' appears to be unused", var),
                    Some(format!("Consider prefixing with underscore: '_{}'", var)),
                );
                self.diagnostics.push(diagnostic);
            }
        }
    }

    pub fn lint_count(&self) -> usize {
        self.diagnostics.len()
    }
}

impl<'ast> Visit<'ast> for FunctionLinter {
    fn visit_local(&mut self, local: &'ast Local) {
        // Track variable declarations
        if let Pat::Ident(pat_ident) = &local.pat {
            self.unused_variables.push(pat_ident.ident.to_string());
        }
        
        // Continue visiting
        syn::visit::visit_local(self, local);
    }

    fn visit_expr(&mut self, expr: &'ast Expr) {
        // Count complex expressions
        match expr {
            Expr::Match(_) | Expr::If(_) | Expr::While(_) | Expr::ForLoop(_) => {
                self.complex_expressions += 1;
            }
            Expr::Call(call) => {
                // Nested function calls add complexity
                if matches!(call.func.as_ref(), Expr::Call(_)) {
                    self.complex_expressions += 1;
                }
            }
            _ => {}
        }
        
        // Continue visiting
        syn::visit::visit_expr(self, expr);
    }
}

/// Linter for analyzing struct definitions
pub struct StructLinter {
    diagnostics: Vec<Diagnostic>,
}

impl StructLinter {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    /// Analyze a struct for potential issues
    pub fn analyze_struct(&mut self, input: &DeriveInput) -> Vec<Diagnostic> {
        self.diagnostics.clear();

        self.check_struct_naming(&input.ident.to_string());
        
        if let syn::Data::Struct(data_struct) = &input.data {
            self.check_field_count(&data_struct.fields);
            self.check_field_naming(&data_struct.fields);
        }

        self.diagnostics.clone()
    }

    fn check_struct_naming(&mut self, name: &str) {
        // Check for PascalCase convention
        if !name.chars().next().unwrap_or('a').is_uppercase() {
            let diagnostic = Diagnostic::new(
                DiagnosticLevel::Error,
                format!("Struct '{}' should use PascalCase naming convention", name),
                Some(format!("Consider renaming to '{}'", to_pascal_case(name))),
            );
            self.diagnostics.push(diagnostic);
        }
    }

    fn check_field_count(&mut self, fields: &syn::Fields) {
        let field_count = fields.len();
        if field_count > 10 {
            let diagnostic = Diagnostic::new(
                DiagnosticLevel::Warning,
                format!("Struct has many fields ({}). Consider breaking it down.", field_count),
                Some("Consider using composition or grouping related fields".to_string()),
            );
            self.diagnostics.push(diagnostic);
        }
    }

    fn check_field_naming(&mut self, fields: &syn::Fields) {
        for field in fields {
            if let Some(ident) = &field.ident {
                let name = ident.to_string();
                if !name.chars().all(|c| c.is_lowercase() || c == '_' || c.is_numeric()) {
                    let diagnostic = Diagnostic::new(
                        DiagnosticLevel::Warning,
                        format!("Field '{}' should use snake_case naming convention", name),
                        Some(format!("Consider renaming to '{}'", to_snake_case(&name))),
                    );
                    self.diagnostics.push(diagnostic);
                }
            }
        }
    }

    pub fn lint_count(&self) -> usize {
        self.diagnostics.len()
    }
}

/// Linter for analyzing entire modules
pub struct ModuleLinter {
    diagnostics: Vec<Diagnostic>,
    function_count: usize,
    struct_count: usize,
}

impl ModuleLinter {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
            function_count: 0,
            struct_count: 0,
        }
    }

    /// Analyze a module item
    pub fn analyze_item(&mut self, item: &Item) -> Vec<Diagnostic> {
        let mut current_diagnostics = Vec::new();

        match item {
            Item::Fn(func) => {
                self.function_count += 1;
                let mut func_linter = FunctionLinter::new();
                current_diagnostics.extend(func_linter.analyze_function(func));
            }
            Item::Struct(_) => {
                self.struct_count += 1;
            }
            _ => {}
        }

        // Check module-level metrics
        if self.function_count > 20 {
            let diagnostic = Diagnostic::new(
                DiagnosticLevel::Info,
                format!("Module has many functions ({}). Consider splitting into submodules.", self.function_count),
                Some("Consider organizing functions into logical submodules".to_string()),
            );
            current_diagnostics.push(diagnostic);
        }

        self.diagnostics.extend(current_diagnostics.clone());
        current_diagnostics
    }

    pub fn lint_count(&self) -> usize {
        self.diagnostics.len()
    }
}

// Helper functions for name conversion
fn to_snake_case(name: &str) -> String {
    let mut result = String::new();
    let mut chars = name.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch.is_uppercase() && !result.is_empty() {
            result.push('_');
        }
        result.push(ch.to_lowercase().next().unwrap_or(ch));
    }
    
    result
}

fn to_pascal_case(name: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;
    
    for ch in name.chars() {
        if ch == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(ch.to_uppercase().next().unwrap_or(ch));
            capitalize_next = false;
        } else {
            result.push(ch);
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_snake_case_conversion() {
        assert_eq!(to_snake_case("CamelCase"), "camel_case");
        assert_eq!(to_snake_case("XMLParser"), "x_m_l_parser");
        assert_eq!(to_snake_case("simple"), "simple");
    }

    #[test]
    fn test_pascal_case_conversion() {
        assert_eq!(to_pascal_case("snake_case"), "SnakeCase");
        assert_eq!(to_pascal_case("simple"), "Simple");
        assert_eq!(to_pascal_case("already_Pascal"), "AlreadyPascal");
    }

    #[test]
    fn test_function_linter() {
        let func: ItemFn = parse_quote! {
            fn BadFunctionName() {
                let unused_var = 42;
                if true {
                    match Some(1) {
                        Some(x) => println!("{}", x),
                        None => {}
                    }
                }
            }
        };

        let mut linter = FunctionLinter::new();
        let diagnostics = linter.analyze_function(&func);
        
        assert!(!diagnostics.is_empty());
        assert!(diagnostics.iter().any(|d| d.message().contains("snake_case")));
    }

    #[test]
    fn test_struct_linter() {
        let input: DeriveInput = parse_quote! {
            struct badStructName {
                BadField: i32,
                another_bad_field: String,
            }
        };

        let mut linter = StructLinter::new();
        let diagnostics = linter.analyze_struct(&input);
        
        assert!(!diagnostics.is_empty());
        assert!(diagnostics.iter().any(|d| d.message().contains("PascalCase")));
    }
}