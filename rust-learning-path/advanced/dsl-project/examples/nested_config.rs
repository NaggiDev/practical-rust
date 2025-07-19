use dsl_project::*;

fn main() {
    println!("=== Nested Configuration DSL Example ===\n");
    
    // Note: This example demonstrates the intended usage of the config! macro
    // The current implementation is simplified and may not fully support
    // all nested features shown here. This serves as a target for full implementation.
    
    println!("Demonstrating nested configuration concepts...\n");
    
    // Example 1: Simulated nested configuration structure
    #[derive(Debug, Clone)]
    struct DatabaseConfig {
        host: &'static str,
        port: u16,
        name: &'static str,
        ssl_enabled: bool,
    }
    
    impl DatabaseConfig {
        fn new() -> Self {
            Self {
                host: "localhost",
                port: 5432,
                name: "myapp_db",
                ssl_enabled: true,
            }
        }
    }
    
    #[derive(Debug, Clone)]
    struct ServerConfig {
        host: &'static str,
        port: u16,
        workers: u8,
        timeout: u32,
    }
    
    impl ServerConfig {
        fn new() -> Self {
            Self {
                host: "0.0.0.0",
                port: 8080,
                workers: 4,
                timeout: 30,
            }
        }
    }
    
    #[derive(Debug, Clone)]
    struct AppConfig {
        name: &'static str,
        version: &'static str,
        debug: bool,
        database: DatabaseConfig,
        server: ServerConfig,
    }
    
    impl AppConfig {
        fn new() -> Self {
            Self {
                name: "MyApp",
                version: "1.0.0",
                debug: true,
                database: DatabaseConfig::new(),
                server: ServerConfig::new(),
            }
        }
    }
    
    // Create and display the nested configuration
    let config = AppConfig::new();
    
    println!("Application Configuration:");
    println!("  Name: {}", config.name);
    println!("  Version: {}", config.version);
    println!("  Debug: {}", config.debug);
    println!();
    
    println!("Database Configuration:");
    println!("  Host: {}", config.database.host);
    println!("  Port: {}", config.database.port);
    println!("  Database: {}", config.database.name);
    println!("  SSL Enabled: {}", config.database.ssl_enabled);
    println!();
    
    println!("Server Configuration:");
    println!("  Host: {}", config.server.host);
    println!("  Port: {}", config.server.port);
    println!("  Workers: {}", config.server.workers);
    println!("  Timeout: {}s", config.server.timeout);
    println!();
    
    // Example 2: Configuration validation for nested structures
    #[derive(ConfigValidate)]
    struct NestedValidationExample {
        app_name: String,
        database_url: String,
        max_connections: u16,
        cache_enabled: bool,
    }
    
    let valid_nested = NestedValidationExample {
        app_name: "NestedApp".to_string(),
        database_url: "postgresql://localhost/db".to_string(),
        max_connections: 100,
        cache_enabled: true,
    };
    
    let invalid_nested = NestedValidationExample {
        app_name: "".to_string(), // Invalid
        database_url: "".to_string(), // Invalid
        max_connections: 0, // Invalid
        cache_enabled: false,
    };
    
    println!("Nested Configuration Validation:");
    println!("  Valid nested config: {}", valid_nested.is_valid());
    println!("  Invalid nested config: {}", invalid_nested.is_valid());
    
    // Example 3: Advanced configuration with environment integration
    advanced_config! {
        struct ProductionConfig {
            name: String = "ProductionApp",
            port: u16 = 8080,
            debug: bool = false,
        }
    }
    
    let prod_config = AdvancedConfig::new();
    println!("\nAdvanced Configuration:");
    println!("  Name: {}", prod_config.name);
    println!("  Port: {}", prod_config.port);
    println!("  Debug: {}", prod_config.debug);
    
    println!("\n=== Nested Configuration Example Complete ===");
    println!("\nNote: This example shows the intended design of the DSL.");
    println!("The full nested syntax would be implemented in the complete version:");
    println!();
    println!("config! {{");
    println!("    app \"MyApp\" {{");
    println!("        version: \"1.0.0\",");
    println!("        debug: true,");
    println!("        ");
    println!("        database {{");
    println!("            host: \"localhost\",");
    println!("            port: 5432,");
    println!("            name: \"myapp_db\"");
    println!("        }},");
    println!("        ");
    println!("        server {{");
    println!("            host: \"0.0.0.0\",");
    println!("            port: 8080,");
    println!("            workers: 4");
    println!("        }}");
    println!("    }}");
    println!("}}");
}