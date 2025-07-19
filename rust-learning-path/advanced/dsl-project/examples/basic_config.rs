use dsl_project::*;

fn main() {
    println!("=== Basic Configuration DSL Example ===\n");
    
    // Example 1: Simple configuration using declarative macro
    simple_config! {
        AppConfig {
            name: "MyApplication",
            version: "1.0.0",
            author: "Rust Developer"
        }
    }
    
    let config = AppConfig::new();
    println!("Application Configuration:");
    println!("  Name: {}", config.name);
    println!("  Version: {}", config.version);
    println!("  Author: {}", config.author);
    println!();
    
    // Example 2: Multiple configurations
    simple_config! {
        DatabaseConfig {
            host: "localhost",
            port: "5432",
            database: "myapp"
        }
    }
    
    simple_config! {
        ServerConfig {
            bind_address: "0.0.0.0",
            port: "8080",
            workers: "4"
        }
    }
    
    let db_config = DatabaseConfig::new();
    let server_config = ServerConfig::new();
    
    println!("Database Configuration:");
    println!("  Host: {}", db_config.host);
    println!("  Port: {}", db_config.port);
    println!("  Database: {}", db_config.database);
    println!();
    
    println!("Server Configuration:");
    println!("  Bind Address: {}", server_config.bind_address);
    println!("  Port: {}", server_config.port);
    println!("  Workers: {}", server_config.workers);
    println!();
    
    // Example 3: Configuration with validation
    #[derive(ConfigValidate)]
    struct ValidatedConfig {
        service_name: String,
        port: u16,
        enabled: bool,
    }
    
    let valid_config = ValidatedConfig {
        service_name: "web-service".to_string(),
        port: 8080,
        enabled: true,
    };
    
    let invalid_config = ValidatedConfig {
        service_name: "".to_string(), // Invalid: empty string
        port: 0,                      // Invalid: zero port
        enabled: false,
    };
    
    println!("Configuration Validation:");
    println!("  Valid config is valid: {}", valid_config.is_valid());
    println!("  Invalid config is valid: {}", invalid_config.is_valid());
    
    match invalid_config.validate() {
        Ok(_) => println!("  Validation passed unexpectedly"),
        Err(msg) => println!("  Validation error: {}", msg),
    }
    
    println!("\n=== Basic Configuration Example Complete ===");
}