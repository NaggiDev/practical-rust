//! Resource management traits and implementations.

use super::{ResourceManager, ResourceHandle, Monitorable, HealthStatus};
use crate::error::{Result, EngineError};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Advanced resource manager with monitoring and optimization
pub trait AdvancedResourceManager: ResourceManager + Monitorable {
    /// Reserve resources for future use
    fn reserve_resources(&self, estimated_cost: u32, duration: Duration) -> Result<ReservationHandle>;

    /// Cancel a resource reservation
    fn cancel_reservation(&self, handle: ReservationHandle) -> Result<()>;

    /// Get resource usage history
    fn usage_history(&self, duration: Duration) -> Vec<ResourceUsagePoint>;

    /// Optimize resource allocation based on usage patterns
    fn optimize(&self) -> Result<OptimizationReport>;

    /// Set resource limits
    fn set_limits(&self, limits: ResourceLimits) -> Result<()>;

    /// Get current resource limits
    fn get_limits(&self) -> ResourceLimits;
}

/// Handle for resource reservations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReservationHandle {
    pub id: u64,
    pub reserved_at: Instant,
    pub expires_at: Instant,
    pub cost: u32,
}

impl ReservationHandle {
    pub fn new(id: u64, cost: u32, duration: Duration) -> Self {
        let now = Instant::now();
        Self {
            id,
            reserved_at: now,
            expires_at: now + duration,
            cost,
        }
    }

    pub fn is_expired(&self) -> bool {
        Instant::now() > self.expires_at
    }

    pub fn time_remaining(&self) -> Duration {
        self.expires_at.saturating_duration_since(Instant::now())
    }
}

/// Point-in-time resource usage data
#[derive(Debug, Clone)]
pub struct ResourceUsagePoint {
    pub timestamp: Instant,
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub io_usage: f64,
    pub network_usage: f64,
    pub active_tasks: usize,
}

/// Resource optimization report
#[derive(Debug, Clone)]
pub struct OptimizationReport {
    pub recommendations: Vec<OptimizationRecommendation>,
    pub potential_savings: ResourceSavings,
    pub confidence: f64,
    pub generated_at: Instant,
}

/// Individual optimization recommendation
#[derive(Debug, Clone)]
pub struct OptimizationRecommendation {
    pub category: OptimizationCategory,
    pub description: String,
    pub impact: OptimizationImpact,
    pub effort: OptimizationEffort,
}

/// Categories of optimization recommendations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptimizationCategory {
    Memory,
    Cpu,
    Io,
    Network,
    Scheduling,
    Configuration,
}

/// Impact level of optimization
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptimizationImpact {
    Low,
    Medium,
    High,
    Critical,
}

/// Effort required for optimization
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptimizationEffort {
    Minimal,
    Low,
    Medium,
    High,
}

/// Potential resource savings
#[derive(Debug, Clone)]
pub struct ResourceSavings {
    pub memory_mb: u32,
    pub cpu_percent: f64,
    pub io_percent: f64,
    pub network_percent: f64,
    pub cost_reduction_percent: f64,
}

/// Resource limits configuration
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    pub max_memory_mb: u32,
    pub max_cpu_percent: f64,
    pub max_io_mbps: u32,
    pub max_network_mbps: u32,
    pub max_concurrent_tasks: usize,
    pub max_queue_size: usize,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory_mb: 1024, // 1GB
            max_cpu_percent: 80.0,
            max_io_mbps: 100,
            max_network_mbps: 100,
            max_concurrent_tasks: 100,
            max_queue_size: 1000,
        }
    }
}

/// Basic resource manager implementation
#[derive(Debug)]
pub struct BasicResourceManager {
    allocated_resources: Mutex<HashMap<u64, ResourceHandle>>,
    reservations: Mutex<HashMap<u64, ReservationHandle>>,
    limits: Mutex<ResourceLimits>,
    usage_history: Mutex<Vec<ResourceUsagePoint>>,
    next_handle_id: AtomicU64,
    total_capacity: AtomicU32,
    allocated_capacity: AtomicU32,
    start_time: Instant,
}

impl BasicResourceManager {
    pub fn new(total_capacity: u32) -> Self {
        Self {
            allocated_resources: Mutex::new(HashMap::new()),
            reservations: Mutex::new(HashMap::new()),
            limits: Mutex::new(ResourceLimits::default()),
            usage_history: Mutex::new(Vec::new()),
            next_handle_id: AtomicU64::new(1),
            total_capacity: AtomicU32::new(total_capacity),
            allocated_capacity: AtomicU32::new(0),
            start_time: Instant::now(),
        }
    }

    fn next_id(&self) -> u64 {
        self.next_handle_id.fetch_add(1, Ordering::Relaxed)
    }

    fn record_usage(&self) {
        let usage_point = ResourceUsagePoint {
            timestamp: Instant::now(),
            cpu_usage: self.utilization(),
            memory_usage: self.utilization(),
            io_usage: 0.0, // Simplified for this example
            network_usage: 0.0, // Simplified for this example
            active_tasks: self.allocated_resources.lock().unwrap().len(),
        };

        let mut history = self.usage_history.lock().unwrap();
        history.push(usage_point);

        // Keep only last hour of data (assuming 1 point per minute)
        if history.len() > 60 {
            history.remove(0);
        }
    }
}

impl ResourceManager for BasicResourceManager {
    fn allocate_resources(&self, task_id: u64, estimated_cost: u32) -> Result<ResourceHandle> {
        let current_allocated = self.allocated_capacity.load(Ordering::Relaxed);
        let total = self.total_capacity.load(Ordering::Relaxed);

        if current_allocated + estimated_cost > total {
            return Err(EngineError::memory(format!(
                "Insufficient resources: need {}, have {} available",
                estimated_cost,
                total - current_allocated
            )));
        }

        let handle = ResourceHandle::new(self.next_id(), task_id);
        
        {
            let mut allocated = self.allocated_resources.lock().unwrap();
            allocated.insert(handle.id, handle);
        }

        self.allocated_capacity.fetch_add(estimated_cost, Ordering::Relaxed);
        self.record_usage();

        Ok(handle)
    }

    fn release_resources(&self, handle: ResourceHandle) -> Result<()> {
        let mut allocated = self.allocated_resources.lock().unwrap();
        
        if let Some(removed_handle) = allocated.remove(&handle.id) {
            // For simplicity, assume all tasks have cost 1
            // In a real implementation, you'd track the actual cost
            self.allocated_capacity.fetch_sub(1, Ordering::Relaxed);
            self.record_usage();
            Ok(())
        } else {
            Err(EngineError::memory(format!(
                "Resource handle {} not found",
                handle.id
            )))
        }
    }

    fn utilization(&self) -> f64 {
        let allocated = self.allocated_capacity.load(Ordering::Relaxed) as f64;
        let total = self.total_capacity.load(Ordering::Relaxed) as f64;
        
        if total > 0.0 {
            allocated / total
        } else {
            0.0
        }
    }

    fn available_capacity(&self) -> u32 {
        let allocated = self.allocated_capacity.load(Ordering::Relaxed);
        let total = self.total_capacity.load(Ordering::Relaxed);
        total.saturating_sub(allocated)
    }
}

impl Monitorable for BasicResourceManager {
    fn metrics(&self) -> HashMap<String, f64> {
        let mut metrics = HashMap::new();
        
        metrics.insert("utilization".to_string(), self.utilization());
        metrics.insert("available_capacity".to_string(), self.available_capacity() as f64);
        metrics.insert("total_capacity".to_string(), self.total_capacity.load(Ordering::Relaxed) as f64);
        metrics.insert("allocated_capacity".to_string(), self.allocated_capacity.load(Ordering::Relaxed) as f64);
        metrics.insert("active_allocations".to_string(), self.allocated_resources.lock().unwrap().len() as f64);
        metrics.insert("uptime_seconds".to_string(), self.start_time.elapsed().as_secs() as f64);

        metrics
    }

    fn health(&self) -> HealthStatus {
        let utilization = self.utilization();
        
        if utilization > 0.95 {
            HealthStatus::Critical {
                message: format!("Resource utilization critical: {:.1}%", utilization * 100.0),
            }
        } else if utilization > 0.80 {
            HealthStatus::Warning {
                message: format!("Resource utilization high: {:.1}%", utilization * 100.0),
            }
        } else {
            HealthStatus::Healthy
        }
    }

    fn status(&self) -> String {
        format!(
            "ResourceManager: {:.1}% utilized, {} active allocations, {} capacity available",
            self.utilization() * 100.0,
            self.allocated_resources.lock().unwrap().len(),
            self.available_capacity()
        )
    }
}

impl AdvancedResourceManager for BasicResourceManager {
    fn reserve_resources(&self, estimated_cost: u32, duration: Duration) -> Result<ReservationHandle> {
        let available = self.available_capacity();
        
        if estimated_cost > available {
            return Err(EngineError::memory(format!(
                "Cannot reserve {} units, only {} available",
                estimated_cost, available
            )));
        }

        let handle = ReservationHandle::new(self.next_id(), estimated_cost, duration);
        
        {
            let mut reservations = self.reservations.lock().unwrap();
            reservations.insert(handle.id, handle);
        }

        Ok(handle)
    }

    fn cancel_reservation(&self, handle: ReservationHandle) -> Result<()> {
        let mut reservations = self.reservations.lock().unwrap();
        
        if reservations.remove(&handle.id).is_some() {
            Ok(())
        } else {
            Err(EngineError::memory(format!(
                "Reservation {} not found",
                handle.id
            )))
        }
    }

    fn usage_history(&self, duration: Duration) -> Vec<ResourceUsagePoint> {
        let history = self.usage_history.lock().unwrap();
        let cutoff = Instant::now() - duration;
        
        history
            .iter()
            .filter(|point| point.timestamp >= cutoff)
            .cloned()
            .collect()
    }

    fn optimize(&self) -> Result<OptimizationReport> {
        let mut recommendations = Vec::new();
        let utilization = self.utilization();

        // Generate recommendations based on current state
        if utilization > 0.8 {
            recommendations.push(OptimizationRecommendation {
                category: OptimizationCategory::Memory,
                description: "Consider increasing total capacity or optimizing task resource usage".to_string(),
                impact: OptimizationImpact::High,
                effort: OptimizationEffort::Medium,
            });
        }

        if utilization < 0.3 {
            recommendations.push(OptimizationRecommendation {
                category: OptimizationCategory::Configuration,
                description: "Resource capacity may be over-provisioned".to_string(),
                impact: OptimizationImpact::Medium,
                effort: OptimizationEffort::Low,
            });
        }

        let potential_savings = ResourceSavings {
            memory_mb: if utilization < 0.5 { 100 } else { 0 },
            cpu_percent: if utilization < 0.5 { 10.0 } else { 0.0 },
            io_percent: 0.0,
            network_percent: 0.0,
            cost_reduction_percent: if utilization < 0.5 { 15.0 } else { 0.0 },
        };

        Ok(OptimizationReport {
            recommendations,
            potential_savings,
            confidence: 0.75,
            generated_at: Instant::now(),
        })
    }

    fn set_limits(&self, limits: ResourceLimits) -> Result<()> {
        let mut current_limits = self.limits.lock().unwrap();
        *current_limits = limits;
        Ok(())
    }

    fn get_limits(&self) -> ResourceLimits {
        self.limits.lock().unwrap().clone()
    }
}