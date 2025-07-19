//! # Diagnostics Module
//!
//! This module handles the creation and emission of diagnostic messages
//! for the compiler plugin. It demonstrates how to create user-friendly
//! error messages and suggestions.

use std::fmt;

/// Represents the severity level of a diagnostic message
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagnosticLevel {
    /// Informational message
    Info,
    /// Warning that doesn't prevent compilation
    Warning,
    /// Error that prevents compilation
    Error,
}

impl fmt::Display for DiagnosticLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiagnosticLevel::Info => write!(f, "info"),
            DiagnosticLevel::Warning => write!(f, "warning"),
            DiagnosticLevel::Error => write!(f, "error"),
        }
    }
}

/// Represents a diagnostic message with optional suggestions
#[derive(Debug, Clone)]
pub struct Diagnostic {
    level: DiagnosticLevel,
    message: String,
    suggestion: Option<String>,
    code: Option<String>,
    span_info: Option<SpanInfo>,
}

/// Information about the location of a diagnostic in source code
#[derive(Debug, Clone)]
pub struct SpanInfo {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub length: usize,
}

impl Diagnostic {
    /// Create a new diagnostic message
    pub fn new(level: DiagnosticLevel, message: String, suggestion: Option<String>) -> Self {
        Self {
            level,
            message,
            suggestion,
            code: None,
            span_info: None,
        }
    }

    /// Create a new diagnostic with a specific error code
    pub fn with_code(mut self, code: String) -> Self {
        self.code = Some(code);
        self
    }

    /// Add span information to the diagnostic
    pub fn with_span(mut self, span: SpanInfo) -> Self {
        self.span_info = Some(span);
        self
    }

    /// Get the diagnostic level
    pub fn level(&self) -> DiagnosticLevel {
        self.level.clone()
    }

    /// Get the diagnostic message
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Get the suggestion, if any
    pub fn suggestion(&self) -> Option<&str> {
        self.suggestion.as_deref()
    }

    /// Get the error code, if any
    pub fn code(&self) -> Option<&str> {
        self.code.as_deref()
    }

    /// Emit the diagnostic message
    /// In a real compiler plugin, this would integrate with the compiler's
    /// diagnostic system. For this example, we'll print to stderr.
    pub fn emit(&self) {
        let level_str = match self.level {
            DiagnosticLevel::Info => "info",
            DiagnosticLevel::Warning => "warning",
            DiagnosticLevel::Error => "error",
        };

        // Format the basic diagnostic message
        let mut output = format!("{}: {}", level_str, self.message);

        // Add error code if present
        if let Some(code) = &self.code {
            output = format!("{}: {} [{}]", level_str, self.message, code);
        }

        // Add span information if present
        if let Some(span) = &self.span_info {
            output = format!("{}:{}:{}: {}", span.file, span.line, span.column, output);
        }

        eprintln!("{}", output);

        // Add suggestion if present
        if let Some(suggestion) = &self.suggestion {
            eprintln!("  = help: {}", suggestion);
        }
    }

    /// Convert to a structured format for testing
    pub fn to_structured(&self) -> StructuredDiagnostic {
        StructuredDiagnostic {
            level: self.level.clone(),
            message: self.message.clone(),
            suggestion: self.suggestion.clone(),
            code: self.code.clone(),
            span: self.span_info.clone(),
        }
    }
}

/// Structured representation of a diagnostic for testing and serialization
#[derive(Debug, Clone, PartialEq)]
pub struct StructuredDiagnostic {
    pub level: DiagnosticLevel,
    pub message: String,
    pub suggestion: Option<String>,
    pub code: Option<String>,
    pub span: Option<SpanInfo>,
}

/// Builder for creating diagnostics with a fluent interface
pub struct DiagnosticBuilder {
    level: DiagnosticLevel,
    message: String,
    suggestion: Option<String>,
    code: Option<String>,
    span_info: Option<SpanInfo>,
}

impl DiagnosticBuilder {
    /// Create a new diagnostic builder
    pub fn new(level: DiagnosticLevel, message: String) -> Self {
        Self {
            level,
            message,
            suggestion: None,
            code: None,
            span_info: None,
        }
    }

    /// Add a suggestion to the diagnostic
    pub fn with_suggestion<S: Into<String>>(mut self, suggestion: S) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }

    /// Add an error code to the diagnostic
    pub fn with_code<S: Into<String>>(mut self, code: S) -> Self {
        self.code = Some(code.into());
        self
    }

    /// Add span information to the diagnostic
    pub fn with_span_info(mut self, span: SpanInfo) -> Self {
        self.span_info = Some(span);
        self
    }

    /// Build the final diagnostic
    pub fn build(self) -> Diagnostic {
        Diagnostic {
            level: self.level,
            message: self.message,
            suggestion: self.suggestion,
            code: self.code,
            span_info: self.span_info,
        }
    }

    /// Build and emit the diagnostic in one step
    pub fn emit(self) {
        self.build().emit();
    }
}

/// Collection of diagnostics with utility methods
pub struct DiagnosticCollection {
    diagnostics: Vec<Diagnostic>,
}

impl DiagnosticCollection {
    /// Create a new empty collection
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    /// Add a diagnostic to the collection
    pub fn add(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    /// Add multiple diagnostics to the collection
    pub fn extend(&mut self, diagnostics: Vec<Diagnostic>) {
        self.diagnostics.extend(diagnostics);
    }

    /// Get all diagnostics
    pub fn all(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    /// Get diagnostics by level
    pub fn by_level(&self, level: DiagnosticLevel) -> Vec<&Diagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| d.level == level)
            .collect()
    }

    /// Check if there are any errors
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.level == DiagnosticLevel::Error)
    }

    /// Get the count of diagnostics by level
    pub fn count_by_level(&self, level: DiagnosticLevel) -> usize {
        self.diagnostics
            .iter()
            .filter(|d| d.level == level)
            .count()
    }

    /// Emit all diagnostics
    pub fn emit_all(&self) {
        for diagnostic in &self.diagnostics {
            diagnostic.emit();
        }
    }

    /// Clear all diagnostics
    pub fn clear(&mut self) {
        self.diagnostics.clear();
    }

    /// Check if the collection is empty
    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    /// Get the total count of diagnostics
    pub fn len(&self) -> usize {
        self.diagnostics.len()
    }
}

impl Default for DiagnosticCollection {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience functions for creating common diagnostic types
pub mod presets {
    use super::*;

    /// Create a warning about naming conventions
    pub fn naming_convention_warning(item_type: &str, name: &str, expected: &str) -> Diagnostic {
        DiagnosticBuilder::new(
            DiagnosticLevel::Warning,
            format!("{} '{}' doesn't follow naming conventions", item_type, name),
        )
        .with_suggestion(format!("Consider renaming to '{}'", expected))
        .with_code("naming-convention".to_string())
        .build()
    }

    /// Create a warning about complexity
    pub fn complexity_warning(item_type: &str, name: &str, metric: usize) -> Diagnostic {
        DiagnosticBuilder::new(
            DiagnosticLevel::Warning,
            format!("{} '{}' has high complexity ({})", item_type, name, metric),
        )
        .with_suggestion("Consider breaking this down into smaller parts".to_string())
        .with_code("high-complexity".to_string())
        .build()
    }

    /// Create an info message about best practices
    pub fn best_practice_info(message: &str, suggestion: &str) -> Diagnostic {
        DiagnosticBuilder::new(DiagnosticLevel::Info, message.to_string())
            .with_suggestion(suggestion.to_string())
            .with_code("best-practice".to_string())
            .build()
    }

    /// Create an error for invalid syntax or usage
    pub fn usage_error(message: &str) -> Diagnostic {
        DiagnosticBuilder::new(DiagnosticLevel::Error, message.to_string())
            .with_code("usage-error".to_string())
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagnostic_creation() {
        let diagnostic = Diagnostic::new(
            DiagnosticLevel::Warning,
            "Test warning".to_string(),
            Some("Test suggestion".to_string()),
        );

        assert_eq!(diagnostic.level(), DiagnosticLevel::Warning);
        assert_eq!(diagnostic.message(), "Test warning");
        assert_eq!(diagnostic.suggestion(), Some("Test suggestion"));
    }

    #[test]
    fn test_diagnostic_builder() {
        let diagnostic = DiagnosticBuilder::new(
            DiagnosticLevel::Error,
            "Test error".to_string(),
        )
        .with_suggestion("Fix this")
        .with_code("E001")
        .build();

        assert_eq!(diagnostic.level(), DiagnosticLevel::Error);
        assert_eq!(diagnostic.message(), "Test error");
        assert_eq!(diagnostic.suggestion(), Some("Fix this"));
        assert_eq!(diagnostic.code(), Some("E001"));
    }

    #[test]
    fn test_diagnostic_collection() {
        let mut collection = DiagnosticCollection::new();
        
        let warning = Diagnostic::new(
            DiagnosticLevel::Warning,
            "Warning message".to_string(),
            None,
        );
        
        let error = Diagnostic::new(
            DiagnosticLevel::Error,
            "Error message".to_string(),
            None,
        );

        collection.add(warning);
        collection.add(error);

        assert_eq!(collection.len(), 2);
        assert!(collection.has_errors());
        assert_eq!(collection.count_by_level(DiagnosticLevel::Warning), 1);
        assert_eq!(collection.count_by_level(DiagnosticLevel::Error), 1);
    }

    #[test]
    fn test_preset_diagnostics() {
        let naming_diag = presets::naming_convention_warning("function", "BadName", "bad_name");
        assert_eq!(naming_diag.level(), DiagnosticLevel::Warning);
        assert!(naming_diag.message().contains("BadName"));
        assert_eq!(naming_diag.code(), Some("naming-convention"));

        let complexity_diag = presets::complexity_warning("function", "complex_func", 10);
        assert_eq!(complexity_diag.level(), DiagnosticLevel::Warning);
        assert!(complexity_diag.message().contains("10"));
        assert_eq!(complexity_diag.code(), Some("high-complexity"));
    }

    #[test]
    fn test_span_info() {
        let span = SpanInfo {
            file: "test.rs".to_string(),
            line: 42,
            column: 10,
            length: 5,
        };

        let diagnostic = Diagnostic::new(
            DiagnosticLevel::Error,
            "Test error".to_string(),
            None,
        ).with_span(span);

        assert!(diagnostic.span_info.is_some());
        let span_info = diagnostic.span_info.unwrap();
        assert_eq!(span_info.file, "test.rs");
        assert_eq!(span_info.line, 42);
        assert_eq!(span_info.column, 10);
    }
}