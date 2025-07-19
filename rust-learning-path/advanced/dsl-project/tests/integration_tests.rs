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

#[test]
fn test_validation_traits() {
    // Test string validation
    assert!("valid".is_valid());
    assert!(!"".is_valid());
    
    // Test numeric validation
    assert!(8080u16.is_valid());
    assert!(!0u16.is_valid());
    
    // Test boolean validation (always valid)
    assert!(true.is_valid());
    assert!(false.is_valid());
}

#[test]
fn test_config_field_attribute() {
    struct TestFieldConfig {
        #[config_field(required = true)]
        name: String,
        #[config_field(min = 1, max = 65535)]
        port: u16,
    }
    
    // This test verifies that the attribute macro doesn't break compilation
    let _config = TestFieldConfig {
        name: "test".to_string(),
        port: 8080,
    };
}

// Macro expansion tests
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

// Error handling tests
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