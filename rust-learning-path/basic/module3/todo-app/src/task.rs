use chrono::{DateTime, Local};
use std::fmt;

/// Represents the status of a task
#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TaskStatus::Pending => write!(f, "Pending"),
            TaskStatus::InProgress => write!(f, "In Progress"),
            TaskStatus::Completed => write!(f, "Completed"),
        }
    }
}

/// Represents a single task in the to-do list
#[derive(Debug, Clone)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub due_date: Option<DateTime<Local>>,
    pub created_at: DateTime<Local>,
}

impl Task {
    /// Creates a new task with the given title
    pub fn new(id: usize, title: String, description: Option<String>, due_date: Option<DateTime<Local>>) -> Self {
        Task {
            id,
            title,
            description,
            status: TaskStatus::Pending,
            due_date,
            created_at: Local::now(),
        }
    }

    /// Updates the title of the task
    pub fn update_title(&mut self, title: String) {
        self.title = title;
    }

    /// Updates the description of the task
    pub fn update_description(&mut self, description: Option<String>) {
        self.description = description;
    }

    /// Updates the status of the task
    pub fn update_status(&mut self, status: TaskStatus) {
        self.status = status;
    }

    /// Updates the due date of the task
    pub fn update_due_date(&mut self, due_date: Option<DateTime<Local>>) {
        self.due_date = due_date;
    }

    /// Marks the task as completed
    pub fn mark_completed(&mut self) {
        self.status = TaskStatus::Completed;
    }

    /// Marks the task as in progress
    pub fn mark_in_progress(&mut self) {
        self.status = TaskStatus::InProgress;
    }

    /// Marks the task as pending
    pub fn mark_pending(&mut self) {
        self.status = TaskStatus::Pending;
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Task #{}: {} [{}]", self.id, self.title, self.status)?;
        
        if let Some(desc) = &self.description {
            write!(f, "\n  Description: {}", desc)?;
        }
        
        if let Some(due) = &self.due_date {
            write!(f, "\n  Due: {}", due.format("%Y-%m-%d %H:%M"))?;
        }
        
        write!(f, "\n  Created: {}", self.created_at.format("%Y-%m-%d %H:%M"))
    }
}