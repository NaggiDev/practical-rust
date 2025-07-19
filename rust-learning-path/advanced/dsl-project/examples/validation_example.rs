use dsl_project::*;

fn main() {
    println!("=== Configuration Validation Example ===\n");
    
    // Example 1: Basic validation with derive macro
    #[derive(ConfigValidate, Debug)]
    struct WebServerConfig {
        server_name: String,
        port: u16,
        ssl_enabled: bool,
        max_connections: u32,
    }
    
    // Valid configuration
    let valid_config = WebServerConfig {
        server_name: "web-server-01".to_string(),
        port: 8443,
        ssl_enabled: true,
        max_connections: 1000,
    };
    
    // Invalid configuration
    let invalid_config = WebServerConfig {
        server_name: "".to_string(), // Invalid: empty string
        port: 0,                     // Invalid: zero port
        ssl_enabled: true,
        max_connections: 0,          // Invalid: zero connections
    };
    
    println!("Web Server Configuration Validation:");
    println!("Valid config: {:?}", valid_config);
    println!("Is valid: {}", valid_config.is_valid());
    
    match valid_config.validate() {
        Ok(_) => println!("✓ Validation passed"),
        Err(msg) => println!("✗ Validation failed: {}", msg),
    }
    println!();
    
    println!("Invalid config: {:?}", invalid_config);
    println!("Is valid: {}", invalid_config.is_valid());
    
    match invalid_config.validate() {
        Ok(_) => println!("✓ Validation passed"),
        Err(msg) => println!("✗ Validation failed: {}", msg),
    }
    println!();
    
    // Example 2: Multiple configuration types with validation
    #[derive(ConfigValidate, Debug)]
    struct DatabaseConfig {
        host: String,
        port: u16,
        database_name: String,
        username: String,
    }
    
    #[derive(ConfigValidate, Debug)]
    struct CacheConfig {
        redis_url: String,
        max_memory: u32,
        ttl_seconds: u32,
        enabled: bool,
    }
    
    let db_config = DatabaseConfig {
        host: "db.example.com".to_string(),
        port: 5432,
        database_name: "production_db".to_string(),
        username: "app_user".to_string(),
    };
    
    let cache_config = CacheConfig {
        redis_url: "redis://localhost:6379".to_string(),
        max_memory: 256,
        ttl_seconds: 3600,
        enabled: true,
    };
    
    let invalid_db_config = DatabaseConfig {
        host: "".to_string(),        // Invalid
        port: 0,                     // Invalid
        database_name: "".to_string(), // Invalid
        username: "valid_user".to_string(),
    };
    
    println!("Database Configuration Validation:");
    validate_and_report("Valid DB Config", &db_config);
    validate_and_report("Invalid DB Config", &invalid_db_config);
    println!();
    
    println!("Cache Configuration Validation:");
    validate_and_report("Cache Config", &cache_config);
    println!();
    
    // Example 3: Custom validation logic demonstration
    println!("Custom Validation Examples:");
    
    // Test individual field validation
    test_field_validation("Valid server name", "web-server-01");
    test_field_validation("Invalid server name", "");
    
    test_port_validation("Valid port", 8080);
    test_port_validation("Invalid port", 0);
    
    test_connection_validation("Valid connections", 100);
    test_connection_validation("Invalid connections", 0);
    
    println!();
    
    // Example 4: Configuration with field attributes (demonstration)
    struct AdvancedConfig {
        #[config_field(required = true, min_length = 1)]
        service_name: String,
        
        #[config_field(min = 1, max = 65535)]
        port: u16,
        
        #[config_field(min = 1, max = 10000)]
        max_connections: u32,
        
        #[config_field(default = true)]
        logging_enabled: bool,
    }
    
    let advanced_config = AdvancedConfig {
        service_name: "advanced-service".to_string(),
        port: 9000,
        max_connections: 500,
        logging_enabled: true,
    };
    
    println!("Advanced Configuration (with field attributes):");
    println!("  Service Name: {}", advanced_config.service_name);
    println!("  Port: {}", advanced_config.port);
    println!("  Max Connections: {}", advanced_config.max_connections);
    println!("  Logging Enabled: {}", advanced_config.logging_enabled);
    println!("  Note: Field attributes are parsed but not yet fully implemented");
    
    println!("\n=== Configuration Validation Example Complete ===");
}

fn validate_and_report<T: ConfigValidate + std::fmt::Debug>(name: &str, config: &T) {
    println!("{}:", name);
    println!("  Config: {:?}", config);
    println!("  Is valid: {}", config.is_valid());
    
    match config.validate() {
        Ok(_) => println!("  ✓ Validation passed"),
        Err(msg) => println!("  ✗ Validation failed: {}", msg),
    }
}

fn test_field_validation(test_name: &str, value: &str) {
    println!("  {}: '{}' -> {}", 
        test_name, 
        value, 
        if value.is_valid() { "✓ Valid" } else { "✗ Invalid" }
    );
}

fn test_port_validation(test_name: &str, port: u16) {
    println!("  {}: {} -> {}", 
        test_name, 
        port, 
        if port.is_valid() { "✓ Valid" } else { "✗ Invalid" }
    );
}

fn test_connection_validation(test_name: &str, connections: u32) {
    println!("  {}: {} -> {}", 
        test_name, 
        connections, 
        if connections.is_valid() { "✓ Valid" } else { "✗ Invalid" }
    );
}