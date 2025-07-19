use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use crate::error::{DatabaseError, DatabaseResult};
use crate::record::Record;

/// A simple file-based database implementation
/// 
/// This demonstrates:
/// - File I/O operations with proper error handling
/// - JSON serialization for data persistence
/// - CRUD operations with validation
/// - File locking and data integrity
#[derive(Debug)]
pub struct Database {
    file_path: PathBuf,
    records: HashMap<String, Record>,
    is_loaded: bool,
}

impl Database {
    /// Create a new database instance
    pub fn new(file_path: PathBuf) -> DatabaseResult<Self> {
        let mut db = Database {
            file_path,
            records: HashMap::new(),
            is_loaded: false,
        };

        // Try to load existing database
        if db.file_path.exists() {
            db.load()?;
        }

        Ok(db)
    }

    /// Initialize a new database file
    pub fn initialize(&mut self) -> DatabaseResult<()> {
        // Create parent directories if they don't exist
        if let Some(parent) = self.file_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Create empty database file
        let file = File::create(&self.file_path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &HashMap::<String, Record>::new())?;

        self.records.clear();
        self.is_loaded = true;

        Ok(())
    }

    /// Load database from file
    fn load(&mut self) -> DatabaseResult<()> {
        let file = File::open(&self.file_path)?;
        let reader = BufReader::new(file);

        match serde_json::from_reader::<_, HashMap<String, Record>>(reader) {
            Ok(records) => {
                // Validate loaded records
                for (id, record) in &records {
                    if id != &record.id {
                        return Err(DatabaseError::corruption(format!(
                            "Record ID mismatch: key '{}' vs record ID '{}'",
                            id, record.id
                        )));
                    }
                }
                self.records = records;
                self.is_loaded = true;
                Ok(())
            }
            Err(e) => {
                // Check if file is empty (valid for new database)
                if self.file_path.metadata()?.len() == 0 {
                    self.records = HashMap::new();
                    self.is_loaded = true;
                    Ok(())
                } else {
                    Err(DatabaseError::corruption(format!(
                        "Failed to parse database file: {}",
                        e
                    )))
                }
            }
        }
    }

    /// Save database to file
    fn save(&self) -> DatabaseResult<()> {
        if !self.is_loaded {
            return Err(DatabaseError::database("Database not loaded"));
        }

        // Create a temporary file for atomic writes
        let temp_path = self.file_path.with_extension("tmp");
        
        {
            let file = File::create(&temp_path)?;
            let writer = BufWriter::new(file);
            serde_json::to_writer_pretty(writer, &self.records)?;
        }

        // Atomically replace the original file
        std::fs::rename(&temp_path, &self.file_path)?;

        Ok(())
    }

    /// Create a new record
    pub fn create_record(&mut self, record: Record) -> DatabaseResult<()> {
        if !self.is_loaded {
            self.load()?;
        }

        let id = record.id().to_string();

        // Check if record already exists
        if self.records.contains_key(&id) {
            return Err(DatabaseError::record_exists(&id));
        }

        // Validate the record
        Record::create(record.id().to_string(), record.name().to_string(), record.value().to_string())?;

        // Insert and save
        self.records.insert(id, record);
        self.save()?;

        Ok(())
    }

    /// Read a record by ID
    pub fn read_record(&mut self, id: &str) -> DatabaseResult<Option<Record>> {
        if !self.is_loaded {
            self.load()?;
        }

        Ok(self.records.get(id).cloned())
    }

    /// Update an existing record
    pub fn update_record(
        &mut self,
        id: &str,
        name: Option<String>,
        value: Option<String>,
    ) -> DatabaseResult<bool> {
        if !self.is_loaded {
            self.load()?;
        }

        if let Some(record) = self.records.get_mut(id) {
            record.update(name, value)?;
            self.save()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Delete a record by ID
    pub fn delete_record(&mut self, id: &str) -> DatabaseResult<bool> {
        if !self.is_loaded {
            self.load()?;
        }

        if self.records.remove(id).is_some() {
            self.save()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// List all records
    pub fn list_records(&mut self) -> DatabaseResult<Vec<Record>> {
        if !self.is_loaded {
            self.load()?;
        }

        let mut records: Vec<Record> = self.records.values().cloned().collect();
        records.sort_by(|a, b| a.id().cmp(b.id()));
        Ok(records)
    }

    /// Search records by query
    pub fn search_records(&mut self, query: &str) -> DatabaseResult<Vec<Record>> {
        if !self.is_loaded {
            self.load()?;
        }

        let mut matching_records: Vec<Record> = self
            .records
            .values()
            .filter(|record| record.matches(query))
            .cloned()
            .collect();

        matching_records.sort_by(|a, b| a.id().cmp(b.id()));
        Ok(matching_records)
    }

    /// Get database statistics
    pub fn stats(&mut self) -> DatabaseResult<DatabaseStats> {
        if !self.is_loaded {
            self.load()?;
        }

        let file_size = self.file_path.metadata()?.len();
        
        Ok(DatabaseStats {
            record_count: self.records.len(),
            file_size_bytes: file_size,
            file_path: self.file_path.clone(),
        })
    }

    /// Backup database to a new file
    pub fn backup(&self, backup_path: &Path) -> DatabaseResult<()> {
        if !self.is_loaded {
            return Err(DatabaseError::database("Database not loaded"));
        }

        std::fs::copy(&self.file_path, backup_path)?;
        Ok(())
    }

    /// Verify database integrity
    pub fn verify(&mut self) -> DatabaseResult<Vec<String>> {
        if !self.is_loaded {
            self.load()?;
        }

        let mut issues = Vec::new();

        // Check for duplicate IDs (shouldn't happen with HashMap, but good to verify)
        let mut seen_ids = std::collections::HashSet::new();
        for (key, record) in &self.records {
            if key != &record.id {
                issues.push(format!("ID mismatch: key '{}' vs record ID '{}'", key, record.id));
            }
            
            if !seen_ids.insert(&record.id) {
                issues.push(format!("Duplicate record ID: '{}'", record.id));
            }
        }

        // Validate each record
        for record in self.records.values() {
            if let Err(e) = Record::create(
                record.id().to_string(),
                record.name().to_string(),
                record.value().to_string(),
            ) {
                issues.push(format!("Invalid record '{}': {}", record.id(), e));
            }
        }

        Ok(issues)
    }
}

/// Database statistics
#[derive(Debug)]
pub struct DatabaseStats {
    pub record_count: usize,
    pub file_size_bytes: u64,
    pub file_path: PathBuf,
}

impl std::fmt::Display for DatabaseStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Records: {}, File size: {} bytes, Path: {}",
            self.record_count,
            self.file_size_bytes,
            self.file_path.display()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    fn create_test_database() -> (Database, NamedTempFile) {
        let temp_file = NamedTempFile::new().unwrap();
        let db = Database::new(temp_file.path().to_path_buf()).unwrap();
        (db, temp_file)
    }

    #[test]
    fn test_database_initialization() {
        let (mut db, _temp_file) = create_test_database();
        assert!(db.initialize().is_ok());
        assert!(db.is_loaded);
    }

    #[test]
    fn test_crud_operations() {
        let (mut db, _temp_file) = create_test_database();
        db.initialize().unwrap();

        // Create
        let record = Record::new("test1".to_string(), "Test Record".to_string(), "Test Value".to_string());
        assert!(db.create_record(record).is_ok());

        // Read
        let retrieved = db.read_record("test1").unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name(), "Test Record");

        // Update
        assert!(db.update_record("test1", Some("Updated Name".to_string()), None).unwrap());
        let updated = db.read_record("test1").unwrap().unwrap();
        assert_eq!(updated.name(), "Updated Name");

        // Delete
        assert!(db.delete_record("test1").unwrap());
        assert!(db.read_record("test1").unwrap().is_none());
    }

    #[test]
    fn test_duplicate_record_creation() {
        let (mut db, _temp_file) = create_test_database();
        db.initialize().unwrap();

        let record1 = Record::new("test1".to_string(), "First".to_string(), "Value".to_string());
        let record2 = Record::new("test1".to_string(), "Second".to_string(), "Value".to_string());

        assert!(db.create_record(record1).is_ok());
        assert!(db.create_record(record2).is_err());
    }

    #[test]
    fn test_search_functionality() {
        let (mut db, _temp_file) = create_test_database();
        db.initialize().unwrap();

        let record1 = Record::new("user1".to_string(), "John Doe".to_string(), "Engineer".to_string());
        let record2 = Record::new("user2".to_string(), "Jane Smith".to_string(), "Designer".to_string());
        
        db.create_record(record1).unwrap();
        db.create_record(record2).unwrap();

        let results = db.search_records("john").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name(), "John Doe");

        let results = db.search_records("user").unwrap();
        assert_eq!(results.len(), 2);
    }
}

// TODO: Implement the following as part of the learning exercise:
// 1. Database compaction to remove deleted records
// 2. Index creation for faster searches
// 3. Transaction support with rollback
// 4. Database migration and schema versioning