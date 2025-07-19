//! # DSL Project - Domain-Specific Language Implementation
//!
//! This crate demonstrates how to create a Domain-Specific Language (DSL) using Rust's
//! macro system. It includes both declarative and procedural macros for configuration
//! management.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Field, Type, Lit, Meta};

/// A simple declarative macro for basic configuration
/// 
/// This macro demonstrates basic token matching and code generation.
/// It creates a struct with the specified fields and values.
/// 
/// # Example
/// 
/// ```rust
/// simple_config! {
///     AppConfig {
///         name: "MyApp",
///         version: "1.0.0",
///         debug: true,
///         port: 8080
///     }
/// }
/// ```
#[macro_export]
macro_rules! simple_config {
    // Match a struct name followed by field definitions
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

/// The main configuration DSL macro
/// 
/// This macro supports nested configurations and generates type-safe structs.
/// It demonstrates advanced token parsing and recursive macro patterns.
/// 
/// # Example
/// 
/// ```rust
/// config! {
///     app "MyApp" {
///         version: "1.0.0",
///         debug: true,
///         
///         database {
///             host: "localhost",
///             port: 5432,
///             name: "myapp_db"
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! config {
    // Entry point: app name with configuration block
    (app $app_name:literal { $($content:tt)* }) => {
        config!(@parse_config AppConfig, $($content)*);
    };
    
    // Parse configuration content
    (@parse_config $struct_name:ident, $($content:tt)*) => {
        config!(@build_struct $struct_name, [], $($content)*);
    };
    
    // Build struct with accumulated fields
    (@build_struct $struct_name:ident, [$($fields:tt)*], $field:ident: $value:expr, $($rest:tt)*) => {
        config!(@build_struct $struct_name, [$($fields)* ($field, $value, simple),], $($rest)*);
    };
    
    // Handle nested configuration blocks
    (@build_struct $struct_name:ident, [$($fields:tt)*], $field:ident { $($nested:tt)* }, $($rest:tt)*) => {
        config!(@create_nested_struct $field, $($nested)*);
        config!(@build_struct $struct_name, [$($fields)* ($field, $field, nested),], $($rest)*);
    };
    
    // Final struct generation
    (@build_struct $struct_name:ident, [$($fields:tt)*],) => {
        config!(@generate_struct $struct_name, $($fields)*);
    };
    
    // Generate nested struct
    (@create_nested_struct $nested_name:ident, $($content:tt)*) => {
        config!(@build_struct $nested_name, [], $($content)*);
    };
    
    // Generate the final struct definition
    (@generate_struct $struct_name:ident, $(($field:ident, $value:expr, simple),)*) => {
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
    
    // Generate struct with nested fields
    (@generate_struct $struct_name:ident, $(($field:ident, $field_type:ident, $field_kind:ident),)*) => {
        #[derive(Debug, Clone)]
        pub struct $struct_name {
            $(pub $field: config!(@field_type $field_type, $field_kind),)*
        }
        
        impl $struct_name {
            pub fn new() -> Self {
                Self {
                    $(
                        $field: config!(@field_default $field_type, $field_kind),
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
    
    // Helper to determine field type
    (@field_type $field_type:ident, simple) => { &'static str };
    (@field_type $field_type:ident, nested) => { $field_type };
    
    // Helper to determine field default
    (@field_default $field_type:expr, simple) => { $field_type };
    (@field_default $field_type:ident, nested) => { $field_type::new() };
}

/// Derive macro for configuration validation
/// 
/// This procedural macro generates validation methods for configuration structs.
/// It demonstrates how to parse struct definitions and generate trait implementations.
/// 
/// # Example
/// 
/// ```rust
/// #[derive(ConfigValidate)]
/// struct DatabaseConfig {
///     host: String,
///     port: u16,
///     name: String,
/// }
/// ```
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
                        let field_type = &field.ty;
                        
                        quote! {
                            // Validate field based on its type
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

/// Attribute macro for field-level configuration
/// 
/// This macro demonstrates attribute parsing and code modification.
/// It can be used to add validation rules to struct fields.
/// 
/// # Example
/// 
/// ```rust
/// struct Config {
///     #[config_field(required = true, min = 1, max = 65535)]
///     port: u16,
/// }
/// ```
#[proc_macro_attribute]
pub fn config_field(args: TokenStream, input: TokenStream) -> TokenStream {
    // For now, this is a placeholder that just returns the input unchanged
    // In a full implementation, this would parse the attributes and modify the field
    let _args = args; // Parse validation rules from args
    input // Return the field unchanged for now
}

/// Function-like procedural macro for advanced configuration
/// 
/// This macro demonstrates complex token parsing and code generation.
/// It provides more flexibility than declarative macros.
/// 
/// # Example
/// 
/// ```rust
/// advanced_config! {
///     struct AppConfig {
///         name: String = env!("APP_NAME", "DefaultApp"),
///         port: u16 = 8080,
///         debug: bool = cfg!(debug_assertions),
///     }
/// }
/// ```
#[proc_macro]
pub fn advanced_config(input: TokenStream) -> TokenStream {
    // This is a simplified implementation
    // A full implementation would parse the entire configuration syntax
    let input_str = input.to_string();
    
    // For demonstration, generate a simple struct
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
                    port: 8080,
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
    
    #[test]
    fn test_validation_trait() {
        let valid_string = "hello".to_string();
        let empty_string = "".to_string();
        let valid_port: u16 = 8080;
        let invalid_port: u16 = 0;
        
        assert!(valid_string.is_valid());
        assert!(!empty_string.is_valid());
        assert!(valid_port.is_valid());
        assert!(!invalid_port.is_valid());
    }
}