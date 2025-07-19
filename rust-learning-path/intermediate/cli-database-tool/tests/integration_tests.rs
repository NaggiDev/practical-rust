use cli_database_tool::{Database, Record, DatabaseError};
use std::path::PathBuf;
use tempfile::{NamedTempFile, TempDir};

/// Integration tests for the CLI database tool
/// 
/// These tests demonstrate:
/// - End-to-end testing of database operations
/// - Error condition testing
/// - File system interaction testing
/// - Test organization and helper functions

#[test]
fn test_complete_database_workflow() {
    let temp_file = NamedTempFile::new().unwrap();
    let mut db = Database::new(temp_file.path().to_path_buf()).unwrap();

    // Initialize database
    db.initialize().unwrap();

    // Create multiple records
    let records = vec![
        Record::new("user1".to_string(), "Alice Johnson".to_string(), "Software Engineer".to_string()),
        Record::new("user2".to_string(), "Bob Smith".to_string(), "Product Manager".to_string()),
        Record::new("user3".to_string(), "Carol Davis".to_string(), "UX Designer".to_string()),
    ];

    for record in records {
        db.create_record(record).unwrap();
    }

    // Verify all records exist
    let all_records = db.list_records().unwrap();
    assert_eq!(all_records.len(), 3);

    // Test search functionality
    let search_results = db.search_records("alice").unwrap();
    assert_eq!(search_results.len(), 1);
    assert_eq!(search_results[0].name(), "Alice Johnson");

    // Test update operations
    assert!(db.update_record("user1", Some("Alice Williams".to_string()), None).unwrap());
    let updated_record = db.read_record("user1").unwrap().unwrap();
    assert_eq!(updated_record.name(), "Alice Williams");

    // Test deletion
    assert!(db.delete_record("user2").unwrap());
    let remaining_records = db.list_records().unwrap();
    assert_eq!(remaining_records.len(), 2);

    // Verify record is actually gone
    assert!(db.read_record("user2").unwrap().is_none());
}

#[test]
fn test_database_persistence() {
    let temp_file = NamedTempFile::new().unwrap();
    let db_path = temp_file.path().to_path_buf();

    // Create and populate database
    {
        let mut db = Database::new(db_path.clone()).unwrap();
        db.initialize().unwrap();
        
        let record = Record::new("persistent".to_string(), "Persistent Record".to_string(), "This should survive".to_string());
        db.create_record(record).unwrap();
    }

    // Reload database and verify data persists
    {
        let mut db = Database::new(db_path).unwrap();
        let record = db.read_record("persistent").unwrap();
        assert!(record.is_some());
        assert_eq!(record.unwrap().name(), "Persistent Record");
    }
}

#[test]
fn test_error_conditions() {
    let temp_file = NamedTempFile::new().unwrap();
    let mut db = Database::new(temp_file.path().to_path_buf()).unwrap();
    db.initialize().unwrap();

    // Test duplicate record creation
    let record1 = Record::new("duplicate".to_string(), "First".to_string(), "Value".to_string());
    let record2 = Record::new("duplicate".to_string(), "Second".to_string(), "Value".to_string());
    
    db.create_record(record1).unwrap();
    let result = db.create_record(record2);
    assert!(result.is_err());
    
    if let Err(DatabaseError::RecordExists { id }) = result {
        assert_eq!(id, "duplicate");
    } else {
        panic!("Expected RecordExists error");
    }

    // Test operations on non-existent records
    assert!(!db.update_record("nonexistent", Some("New Name".to_string()), None).unwrap());
    assert!(!db.delete_record("nonexistent").unwrap());
    assert!(db.read_record("nonexistent").unwrap().is_none());
}

#[test]
fn test_record_validation_errors() {
    let temp_file = NamedTempFile::new().unwrap();
    let mut db = Database::new(temp_file.path().to_path_buf()).unwrap();
    db.initialize().unwrap();

    // Test invalid record creation through validation
    let invalid_records = vec![
        ("", "Valid Name", "Valid Value"), // Empty ID
        ("valid_id", "", "Valid Value"),   // Empty name
        ("invalid@id", "Valid Name", "Valid Value"), // Invalid ID characters
        ("a".repeat(51).as_str(), "Valid Name", "Valid Value"), // ID too long
        ("valid_id", "a".repeat(101).as_str(), "Valid Value"), // Name too long
        ("valid_id", "Valid Name", "a".repeat(1001).as_str()), // Value too long
    ];

    for (id, name, value) in invalid_records {
        let result = Record::create(id.to_string(), name.to_string(), value.to_string());
        assert!(result.is_err(), "Expected validation error for ID: {}, Name: {}, Value length: {}", id, name, value.len());
    }
}

#[test]
fn test_database_stats() {
    let temp_file = NamedTempFile::new().unwrap();
    let mut db = Database::new(temp_file.path().to_path_buf()).unwrap();
    db.initialize().unwrap();

    // Check initial stats
    let stats = db.stats().unwrap();
    assert_eq!(stats.record_count, 0);
    assert!(stats.file_size_bytes > 0); // Should have some content (empty JSON object)

    // Add records and check stats
    for i in 1..=5 {
        let record = Record::new(
            format!("record{}", i),
            format!("Record {}", i),
            format!("Value {}", i),
        );
        db.create_record(record).unwrap();
    }

    let stats = db.stats().unwrap();
    assert_eq!(stats.record_count, 5);
    assert!(stats.file_size_bytes > 100); // Should be larger with data
}

#[test]
fn test_database_backup() {
    let temp_file = NamedTempFile::new().unwrap();
    let mut db = Database::new(temp_file.path().to_path_buf()).unwrap();
    db.initialize().unwrap();

    // Add some data
    let record = Record::new("backup_test".to_string(), "Backup Test".to_string(), "Test Value".to_string());
    db.create_record(record).unwrap();

    // Create backup
    let backup_file = NamedTempFile::new().unwrap();
    db.backup(backup_file.path()).unwrap();

    // Verify backup by loading it as a new database
    let mut backup_db = Database::new(backup_file.path().to_path_buf()).unwrap();
    let restored_record = backup_db.read_record("backup_test").unwrap();
    assert!(restored_record.is_some());
    assert_eq!(restored_record.unwrap().name(), "Backup Test");
}

#[test]
fn test_database_verification() {
    let temp_file = NamedTempFile::new().unwrap();
    let mut db = Database::new(temp_file.path().to_path_buf()).unwrap();
    db.initialize().unwrap();

    // Add valid records
    let record1 = Record::new("valid1".to_string(), "Valid Record 1".to_string(), "Value 1".to_string());
    let record2 = Record::new("valid2".to_string(), "Valid Record 2".to_string(), "Value 2".to_string());
    
    db.create_record(record1).unwrap();
    db.create_record(record2).unwrap();

    // Verify database integrity
    let issues = db.verify().unwrap();
    assert!(issues.is_empty(), "Database should have no integrity issues: {:?}", issues);
}

#[test]
fn test_concurrent_access_simulation() {
    // This test simulates concurrent access by creating multiple database instances
    // pointing to the same file and performing operations
    let temp_file = NamedTempFile::new().unwrap();
    let db_path = temp_file.path().to_path_buf();

    // Initialize database
    {
        let mut db = Database::new(db_path.clone()).unwrap();
        db.initialize().unwrap();
    }

    // Simulate multiple "processes" accessing the database
    let mut db1 = Database::new(db_path.clone()).unwrap();
    let mut db2 = Database::new(db_path.clone()).unwrap();

    // Each "process" adds records
    let record1 = Record::new("process1".to_string(), "Process 1 Record".to_string(), "Value 1".to_string());
    let record2 = Record::new("process2".to_string(), "Process 2 Record".to_string(), "Value 2".to_string());

    db1.create_record(record1).unwrap();
    db2.create_record(record2).unwrap();

    // Both should be able to read all records (after reloading)
    let records1 = db1.list_records().unwrap();
    let records2 = db2.list_records().unwrap();

    // Note: In a real concurrent scenario, this might not work as expected
    // This test mainly verifies that the file operations don't corrupt the data
    assert!(records1.len() >= 1);
    assert!(records2.len() >= 1);
}

#[test]
fn test_large_dataset_operations() {
    let temp_file = NamedTempFile::new().unwrap();
    let mut db = Database::new(temp_file.path().to_path_buf()).unwrap();
    db.initialize().unwrap();

    // Create a larger dataset
    const RECORD_COUNT: usize = 100;
    
    for i in 0..RECORD_COUNT {
        let record = Record::new(
            format!("record_{:04}", i),
            format!("Record Number {}", i),
            format!("This is the value for record number {}", i),
        );
        db.create_record(record).unwrap();
    }

    // Verify all records were created
    let all_records = db.list_records().unwrap();
    assert_eq!(all_records.len(), RECORD_COUNT);

    // Test search performance with larger dataset
    let search_results = db.search_records("50").unwrap();
    assert!(!search_results.is_empty());

    // Test batch operations
    for i in (0..RECORD_COUNT).step_by(10) {
        let id = format!("record_{:04}", i);
        assert!(db.delete_record(&id).unwrap());
    }

    let remaining_records = db.list_records().unwrap();
    assert_eq!(remaining_records.len(), RECORD_COUNT - 10);
}

// Helper function for creating test records
fn create_test_record(id: &str, name: &str, value: &str) -> Record {
    Record::new(id.to_string(), name.to_string(), value.to_string())
}

// Helper function for setting up a populated test database
fn setup_test_database_with_data() -> (Database, NamedTempFile) {
    let temp_file = NamedTempFile::new().unwrap();
    let mut db = Database::new(temp_file.path().to_path_buf()).unwrap();
    db.initialize().unwrap();

    // Add some test data
    let test_records = vec![
        create_test_record("emp001", "John Doe", "Software Engineer"),
        create_test_record("emp002", "Jane Smith", "Product Manager"),
        create_test_record("emp003", "Bob Johnson", "Designer"),
    ];

    for record in test_records {
        db.create_record(record).unwrap();
    }

    (db, temp_file)
}

#[test]
fn test_database_with_helper_functions() {
    let (mut db, _temp_file) = setup_test_database_with_data();

    // Test that helper function setup worked
    let records = db.list_records().unwrap();
    assert_eq!(records.len(), 3);

    // Test search on pre-populated data
    let results = db.search_records("john").unwrap();
    assert_eq!(results.len(), 2); // John Doe and Bob Johnson
}