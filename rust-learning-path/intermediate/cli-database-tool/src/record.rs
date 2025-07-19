use serde::{Deserialize, Serialize};
use std::fmt;

use crate::error::{DatabaseError, DatabaseResult};

/// A database record with ID, name, and value
/// 
/// This demonstrates:
/// - Struct design with validation
/// - Serialization with serde
/// - Custom Display implementation
/// - Input validation patterns
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Record {
    pub id: String,
    pub name: String,
    pub value: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Record {
    /// Create a new record with validation
    pub fn new(id: String, name: String, value: String) -> Record {
        let now = chrono::Utc::now();
        Record {
            id,
            name,
            value,
            created_at: now,
            updated_at: now,
        }
    }

    /// Create a record with validation
    pub fn create(id: String, name: String, value: String) -> DatabaseResult<Record> {
        // Validate ID
        if id.trim().is_empty() {
            return Err(DatabaseError::invalid_record("id", "ID cannot be empty"));
        }
        if id.len() > 50 {
            return Err(DatabaseError::invalid_record("id", "ID cannot exceed 50 characters"));
        }
        if !id.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
            return Err(DatabaseError::invalid_record(
                "id",
                "ID can only contain alphanumeric characters, underscores, and hyphens",
            ));
        }

        // Validate name
        if name.trim().is_empty() {
            return Err(DatabaseError::invalid_record("name", "Name cannot be empty"));
        }
        if name.len() > 100 {
            return Err(DatabaseError::invalid_record("name", "Name cannot exceed 100 characters"));
        }

        // Validate value
        if value.len() > 1000 {
            return Err(DatabaseError::invalid_record("value", "Value cannot exceed 1000 characters"));
        }

        Ok(Record::new(id.trim().to_string(), name.trim().to_string(), value))
    }

    /// Update the record with new values
    pub fn update(&mut self, name: Option<String>, value: Option<String>) -> DatabaseResult<()> {
        if let Some(new_name) = name {
            if new_name.trim().is_empty() {
                return Err(DatabaseError::invalid_record("name", "Name cannot be empty"));
            }
            if new_name.len() > 100 {
                return Err(DatabaseError::invalid_record("name", "Name cannot exceed 100 characters"));
            }
            self.name = new_name.trim().to_string();
        }

        if let Some(new_value) = value {
            if new_value.len() > 1000 {
                return Err(DatabaseError::invalid_record("value", "Value cannot exceed 1000 characters"));
            }
            self.value = new_value;
        }

        self.updated_at = chrono::Utc::now();
        Ok(())
    }

    /// Get the record ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get the record name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the record value
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Check if the record matches a search query
    pub fn matches(&self, query: &str) -> bool {
        let query = query.to_lowercase();
        self.id.to_lowercase().contains(&query)
            || self.name.to_lowercase().contains(&query)
            || self.value.to_lowercase().contains(&query)
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ID: {}, Name: {}, Value: {}, Created: {}, Updated: {}",
            self.id,
            self.name,
            self.value,
            self.created_at.format("%Y-%m-%d %H:%M:%S UTC"),
            self.updated_at.format("%Y-%m-%d %H:%M:%S UTC")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_creation() {
        let record = Record::new("test_id".to_string(), "Test Name".to_string(), "Test Value".to_string());
        assert_eq!(record.id, "test_id");
        assert_eq!(record.name, "Test Name");
        assert_eq!(record.value, "Test Value");
    }

    #[test]
    fn test_record_validation() {
        // Valid record
        assert!(Record::create("valid_id".to_string(), "Valid Name".to_string(), "Valid Value".to_string()).is_ok());

        // Invalid ID - empty
        assert!(Record::create("".to_string(), "Name".to_string(), "Value".to_string()).is_err());

        // Invalid ID - too long
        let long_id = "a".repeat(51);
        assert!(Record::create(long_id, "Name".to_string(), "Value".to_string()).is_err());

        // Invalid ID - special characters
        assert!(Record::create("invalid@id".to_string(), "Name".to_string(), "Value".to_string()).is_err());

        // Invalid name - empty
        assert!(Record::create("id".to_string(), "".to_string(), "Value".to_string()).is_err());

        // Invalid name - too long
        let long_name = "a".repeat(101);
        assert!(Record::create("id".to_string(), long_name, "Value".to_string()).is_err());

        // Invalid value - too long
        let long_value = "a".repeat(1001);
        assert!(Record::create("id".to_string(), "Name".to_string(), long_value).is_err());
    }

    #[test]
    fn test_record_update() {
        let mut record = Record::new("test".to_string(), "Original".to_string(), "Original Value".to_string());
        let original_created = record.created_at;

        // Update name only
        assert!(record.update(Some("Updated Name".to_string()), None).is_ok());
        assert_eq!(record.name, "Updated Name");
        assert_eq!(record.value, "Original Value");
        assert!(record.updated_at > original_created);

        // Update value only
        assert!(record.update(None, Some("Updated Value".to_string())).is_ok());
        assert_eq!(record.name, "Updated Name");
        assert_eq!(record.value, "Updated Value");

        // Invalid update
        assert!(record.update(Some("".to_string()), None).is_err());
    }

    #[test]
    fn test_record_search() {
        let record = Record::new("test_id".to_string(), "Test Name".to_string(), "Test Value".to_string());

        assert!(record.matches("test"));
        assert!(record.matches("Test"));
        assert!(record.matches("name"));
        assert!(record.matches("value"));
        assert!(!record.matches("nonexistent"));
    }
}

// TODO: Implement the following as part of the learning exercise:
// 1. Additional validation rules (email format, phone numbers, etc.)
// 2. Record versioning and history tracking
// 3. Custom serialization formats
// 4. Record relationships and references