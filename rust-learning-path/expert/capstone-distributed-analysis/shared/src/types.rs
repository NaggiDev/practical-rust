use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Unique identifier for analysis jobs
pub type JobId = Uuid;

/// Unique identifier for worker nodes
pub type WorkerId = String;

/// Analysis job submitted by clients
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisJob {
    pub id: JobId,
    pub project_path: String,
    pub analysis_config: AnalysisConfig,
    pub priority: JobPriority,
    pub created_at: DateTime<Utc>,
    pub timeout_seconds: Option<u64>,
}

/// Configuration for analysis parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig {
    pub include_performance: bool,
    pub include_security: bool,
    pub include_style: bool,
    pub custom_lints: Vec<String>,
    pub max_file_size_mb: u64,
    pub exclude_patterns: Vec<String>,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            include_performance: true,
            include_security: true,
            include_style: true,
            custom_lints: Vec::new(),
            max_file_size_mb: 10,
            exclude_patterns: vec!["target/".to_string(), "*.lock".to_string()],
        }
    }
}

/// Priority levels for analysis jobs
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum JobPriority {
    Low = 1,
    Normal = 2,
    High = 3,
    Critical = 4,
}

/// Current status of an analysis job
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum JobStatus {
    Queued,
    Running { worker_id: WorkerId, progress: f32 },
    Completed { result: AnalysisResult },
    Failed { error: String },
    Cancelled,
}

/// Comprehensive analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub job_id: JobId,
    pub completed_at: DateTime<Utc>,
    pub duration_ms: u64,
    pub files_analyzed: u32,
    pub lines_of_code: u64,
    pub metrics: CodeMetrics,
    pub issues: Vec<CodeIssue>,
    pub performance_insights: Vec<PerformanceInsight>,
    pub security_findings: Vec<SecurityFinding>,
}

/// Code quality metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeMetrics {
    pub cyclomatic_complexity: f64,
    pub maintainability_index: f64,
    pub technical_debt_ratio: f64,
    pub test_coverage: Option<f64>,
    pub documentation_coverage: f64,
    pub dependency_count: u32,
}

/// Individual code issue found during analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeIssue {
    pub severity: IssueSeverity,
    pub category: IssueCategory,
    pub message: String,
    pub file_path: String,
    pub line_number: u32,
    pub column: u32,
    pub suggestion: Option<String>,
    pub rule_id: String,
}

/// Severity levels for code issues
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum IssueSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Categories of code issues
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueCategory {
    Style,
    Performance,
    Security,
    Correctness,
    Complexity,
    Documentation,
}

/// Performance-related insights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceInsight {
    pub insight_type: PerformanceInsightType,
    pub description: String,
    pub file_path: String,
    pub line_number: u32,
    pub estimated_impact: PerformanceImpact,
    pub suggestion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceInsightType {
    AllocationHotspot,
    UnneccessaryClone,
    InefficientIteration,
    BlockingOperation,
    LargeStackAllocation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceImpact {
    Low,
    Medium,
    High,
    Critical,
}

/// Security-related findings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityFinding {
    pub finding_type: SecurityFindingType,
    pub severity: SecuritySeverity,
    pub description: String,
    pub file_path: String,
    pub line_number: u32,
    pub cwe_id: Option<u32>,
    pub remediation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityFindingType {
    UnsafeCode,
    UnvalidatedInput,
    WeakCryptography,
    PathTraversal,
    SqlInjection,
    BufferOverflow,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecuritySeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Worker node information and capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerInfo {
    pub id: WorkerId,
    pub hostname: String,
    pub capabilities: WorkerCapabilities,
    pub current_load: f32,
    pub last_heartbeat: DateTime<Utc>,
    pub status: WorkerStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerCapabilities {
    pub max_concurrent_jobs: u32,
    pub supported_analysis_types: Vec<String>,
    pub cpu_cores: u32,
    pub memory_gb: u32,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorkerStatus {
    Available,
    Busy,
    Maintenance,
    Offline,
}

/// System-wide statistics and health metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStats {
    pub total_jobs_processed: u64,
    pub active_jobs: u32,
    pub queued_jobs: u32,
    pub active_workers: u32,
    pub average_job_duration_ms: f64,
    pub system_load: f32,
    pub uptime_seconds: u64,
}

/// Configuration for the distributed system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    pub coordinator_address: String,
    pub worker_timeout_seconds: u64,
    pub job_timeout_seconds: u64,
    pub max_queue_size: u32,
    pub heartbeat_interval_seconds: u64,
    pub cleanup_interval_seconds: u64,
}