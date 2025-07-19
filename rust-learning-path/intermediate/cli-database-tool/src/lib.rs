//! CLI Database Tool Library
//! 
//! A simple file-based database tool demonstrating error handling patterns,
//! data persistence, and command-line interface development in Rust.

pub mod database;
pub mod errors;
pub mod models;

pub use database::Database;
pub use errors::{DatabaseError, Result};
pub use models::User;