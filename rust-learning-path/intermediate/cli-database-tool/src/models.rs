//! Data models for the database tool
//! 
//! This module defines the data structures that represent records
//! in our database, along with validation logic.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::{DatabaseError, Result};

/// Represents a user record in the database
/// 
/// This struct demonstrates:
/// - Serialization/deserialization with serde
/// - Data validation methods
/// - Working with UUIDs for unique identifiers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    /// Unique identifier for the user
    pub id: Uuid,
    /// User's full name
    pub name: String,
    /// User's email address
    pub email: String,
    /// User's age
    pub age: u32,
}

impl User {
    /// Create a new user with validation
    /// 
    /// This method demonstrates input validation with custom error types.
    /// It checks that all required fields are valid before creating the user.
    pub fn new(name: String, email: String, age: u32) -> Result<Self> {
        // Validate name
        if name.trim().is_empty() {
            return Err(DatabaseError::validation("name", "Name cannot be empty"));
        }

        if name.len() > 100 {
            return Err(DatabaseError::validation("name", "Name cannot exceed 100 characters"));
        }

        // Validate email (basic validation)
        if !email.contains('@') || !email.contains('.') {
            return Err(DatabaseError::validation("email", "Invalid email format"));
        }

        if email.len() > 255 {
            return Err(DatabaseError::validation("email", "Email cannot exceed 255 characters"));
        }

        // Validate age
        if age > 150 {
            return Err(DatabaseError::validation("age", "Age must be realistic (0-150)"));
        }

        Ok(User {
            id: Uuid::new_v4(),
            name: name.trim().to_string(),
            email: email.trim().to_lowercase(),
            age,
        })
    }

    /// Update user fields with validation
    /// 
    /// This method allows updating specific fields while maintaining validation.
    /// It demonstrates how to handle optional updates with error handling.
    pub fn update(
        &mut self,
        name: Option<String>,
        email: Option<String>,
        age: Option<u32>,
    ) -> Result<()> {
        // Update name if provided
        if let Some(new_name) = name {
            if new_name.trim().is_empty() {
                return Err(DatabaseError::validation("name", "Name cannot be empty"));
            }
            if new_name.len() > 100 {
                return Err(DatabaseError::validation("name", "Name cannot exceed 100 characters"));
            }
            self.name = new_name.trim().to_string();
        }

        // Update email if provided
        if let Some(new_email) = email {
            if !new_email.contains('@') || !new_email.contains('.') {
                return Err(DatabaseError::validation("email", "Invalid email format"));
            }
            if new_email.len() > 255 {
                return Err(DatabaseError::validation("email", "Email cannot exceed 255 characters"));
            }
            self.email = new_email.trim().to_lowercase();
        }

        // Update age if provided
        if let Some(new_age) = age {
            if new_age > 150 {
                return Err(DatabaseError::validation("age", "Age must be realistic (0-150)"));
            }
            self.age = new_age;
        }

        Ok(())
    }

    /// Validate an existing user record
    /// 
    /// This method can be used to validate user records loaded from storage
    /// to ensure data integrity.
    pub fn validate(&self) -> Result<()> {
        if self.name.trim().is_empty() || self.name.len() > 100 {
            return Err(DatabaseError::validation("name", "Invalid name"));
        }

        if !self.email.contains('@') || !self.email.contains('.') || self.email.len() > 255 {
            return Err(DatabaseError::validation("email", "Invalid email"));
        }

        if self.age > 150 {
            return Err(DatabaseError::validation("age", "Invalid age"));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation_valid() {
        let user = User::new(
            "John Doe".to_string(),
            "john@example.com".to_string(),
            30,
        );
        assert!(user.is_ok());
        
        let user = user.unwrap();
        assert_eq!(user.name, "John Doe");
        assert_eq!(user.email, "john@example.com");
        assert_eq!(user.age, 30);
    }

    #[test]
    fn test_user_creation_invalid_name() {
        let user = User::new("".to_string(), "john@example.com".to_string(), 30);
        assert!(user.is_err());
        assert!(matches!(user.unwrap_err(), DatabaseError::ValidationError { .. }));
    }

    #[test]
    fn test_user_creation_invalid_email() {
        let user = User::new("John Doe".to_string(), "invalid-email".to_string(), 30);
        assert!(user.is_err());
        assert!(matches!(user.unwrap_err(), DatabaseError::ValidationError { .. }));
    }

    #[test]
    fn test_user_creation_invalid_age() {
        let user = User::new("John Doe".to_string(), "john@example.com".to_string(), 200);
        assert!(user.is_err());
        assert!(matches!(user.unwrap_err(), DatabaseError::ValidationError { .. }));
    }

    #[test]
    fn test_user_update_valid() {
        let mut user = User::new(
            "John Doe".to_string(),
            "john@example.com".to_string(),
            30,
        ).unwrap();

        let result = user.update(
            Some("Jane Doe".to_string()),
            Some("jane@example.com".to_string()),
            Some(25),
        );
        
        assert!(result.is_ok());
        assert_eq!(user.name, "Jane Doe");
        assert_eq!(user.email, "jane@example.com");
        assert_eq!(user.age, 25);
    }

    #[test]
    fn test_user_update_partial() {
        let mut user = User::new(
            "John Doe".to_string(),
            "john@example.com".to_string(),
            30,
        ).unwrap();

        let result = user.update(Some("Jane Doe".to_string()), None, None);
        
        assert!(result.is_ok());
        assert_eq!(user.name, "Jane Doe");
        assert_eq!(user.email, "john@example.com"); // unchanged
        assert_eq!(user.age, 30); // unchanged
    }

    #[test]
    fn test_user_serialization() {
        let user = User::new(
            "John Doe".to_string(),
            "john@example.com".to_string(),
            30,
        ).unwrap();

        let json = serde_json::to_string(&user).unwrap();
        let deserialized: User = serde_json::from_str(&json).unwrap();
        
        assert_eq!(user, deserialized);
    }
}