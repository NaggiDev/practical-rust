use serde::{Deserialize, Serialize};
use crate::types::*;

/// Messages sent from coordinator to workers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoordinatorMessage {
    /// Assign a new analysis job to the worker
    AssignJob {
        job: AnalysisJob,
        chunk_info: Option<ChunkInfo>,
    },
    /// Cancel a running job
    CancelJob { job_id: JobId },
    /// Request worker status update
    StatusRequest,
    /// Shutdown the worker gracefully
    Shutdown,
    /// Update worker configuration
    UpdateConfig { config: WorkerConfig },
}

/// Messages sent from workers to coordinator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkerMessage {
    /// Worker registration with capabilities
    Register { worker_info: WorkerInfo },
    /// Periodic heartbeat with current status
    Heartbeat { 
        worker_id: WorkerId,
        status: WorkerStatus,
        current_load: f32,
        active_jobs: Vec<JobId>,
    },
    /// Job progress update
    JobProgress {
        job_id: JobId,
        progress: f32,
        intermediate_results: Option<IntermediateResults>,
    },
    /// Job completion notification
    JobCompleted {
        job_id: JobId,
        result: AnalysisResult,
    },
    /// Job failure notification
    JobFailed {
        job_id: JobId,
        error: String,
        partial_results: Option<AnalysisResult>,
    },
    /// Worker is shutting down
    Shutdown { worker_id: WorkerId },
}

/// Messages sent from clients to coordinator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    /// Submit a new analysis job
    SubmitJob { job: AnalysisJob },
    /// Query job status
    GetJobStatus { job_id: JobId },
    /// Cancel a job
    CancelJob { job_id: JobId },
    /// Get system statistics
    GetSystemStats,
    /// List all jobs (with optional filters)
    ListJobs { 
        status_filter: Option<JobStatus>,
        limit: Option<u32>,
        offset: Option<u32>,
    },
    /// Subscribe to job updates
    Subscribe { job_id: JobId },
    /// Unsubscribe from job updates
    Unsubscribe { job_id: JobId },
}

/// Messages sent from coordinator to clients
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoordinatorResponse {
    /// Job submission acknowledgment
    JobSubmitted { job_id: JobId },
    /// Job status response
    JobStatus { 
        job_id: JobId,
        status: JobStatus,
    },
    /// Job cancellation acknowledgment
    JobCancelled { job_id: JobId },
    /// System statistics response
    SystemStats { stats: SystemStats },
    /// Job list response
    JobList { 
        jobs: Vec<(JobId, JobStatus)>,
        total_count: u32,
    },
    /// Subscription acknowledgment
    Subscribed { job_id: JobId },
    /// Real-time job update
    JobUpdate {
        job_id: JobId,
        status: JobStatus,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    /// Error response
    Error { message: String },
}

/// Information about job chunks for distributed processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkInfo {
    pub chunk_id: u32,
    pub total_chunks: u32,
    pub file_paths: Vec<String>,
    pub dependencies: Vec<String>,
}

/// Intermediate results during job processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntermediateResults {
    pub files_processed: u32,
    pub issues_found: u32,
    pub current_file: Option<String>,
    pub estimated_completion: Option<chrono::DateTime<chrono::Utc>>,
}

/// Worker-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerConfig {
    pub max_concurrent_jobs: u32,
    pub analysis_timeout_seconds: u64,
    pub memory_limit_mb: u64,
    pub enable_performance_analysis: bool,
    pub enable_security_analysis: bool,
    pub custom_lint_rules: Vec<String>,
}

impl Default for WorkerConfig {
    fn default() -> Self {
        Self {
            max_concurrent_jobs: 4,
            analysis_timeout_seconds: 300,
            memory_limit_mb: 2048,
            enable_performance_analysis: true,
            enable_security_analysis: true,
            custom_lint_rules: Vec::new(),
        }
    }
}

/// Network protocol for efficient binary communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkFrame {
    pub message_type: MessageType,
    pub payload: Vec<u8>,
    pub compression: CompressionType,
    pub checksum: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    CoordinatorToWorker,
    WorkerToCoordinator,
    ClientToCoordinator,
    CoordinatorToClient,
    Heartbeat,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompressionType {
    None,
    Gzip,
    Lz4,
}

/// WebSocket message types for real-time communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebSocketMessage {
    Subscribe { job_ids: Vec<JobId> },
    Unsubscribe { job_ids: Vec<JobId> },
    JobUpdate { 
        job_id: JobId,
        status: JobStatus,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    SystemUpdate { stats: SystemStats },
    Error { message: String },
}

/// HTTP API endpoints and request/response types
pub mod http {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubmitJobRequest {
        pub project_path: String,
        pub config: AnalysisConfig,
        pub priority: Option<JobPriority>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubmitJobResponse {
        pub job_id: JobId,
        pub estimated_duration_seconds: Option<u64>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct JobStatusResponse {
        pub job_id: JobId,
        pub status: JobStatus,
        pub created_at: chrono::DateTime<chrono::Utc>,
        pub updated_at: chrono::DateTime<chrono::Utc>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ListJobsResponse {
        pub jobs: Vec<JobStatusResponse>,
        pub total_count: u32,
        pub page: u32,
        pub per_page: u32,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SystemStatsResponse {
        pub stats: SystemStats,
        pub workers: Vec<WorkerInfo>,
        pub recent_jobs: Vec<JobStatusResponse>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ErrorResponse {
        pub error: String,
        pub code: u32,
        pub details: Option<serde_json::Value>,
    }
}