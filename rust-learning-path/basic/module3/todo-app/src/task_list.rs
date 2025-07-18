use crate::task::{Task, TaskStatus};

/// Manages a collection of tasks
pub struct TaskList {
    tasks: Vec<Task>,
    next_id: usize,
}

impl TaskList {
    /// Creates a new, empty task list
    pub fn new() -> Self {
        TaskList {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    /// Adds a new task to the list
    pub fn add_task(&mut self, title: String, description: Option<String>, due_date: Option<chrono::DateTime<chrono::Local>>) -> &Task {
        let id = self.next_id;
        self.next_id += 1;
        
        let task = Task::new(id, title, description, due_date);
        self.tasks.push(task);
        
        // Return a reference to the newly added task
        self.tasks.last().unwrap()
    }

    /// Gets a task by its ID
    pub fn get_task(&self, id: usize) -> Option<&Task> {
        self.tasks.iter().find(|task| task.id == id)
    }

    /// Gets a mutable reference to a task by its ID
    pub fn get_task_mut(&mut self, id: usize) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|task| task.id == id)
    }

    /// Removes a task by its ID
    pub fn remove_task(&mut self, id: usize) -> bool {
        let position = self.tasks.iter().position(|task| task.id == id);
        
        if let Some(pos) = position {
            self.tasks.remove(pos);
            true
        } else {
            false
        }
    }

    /// Returns all tasks
    pub fn list_all(&self) -> &[Task] {
        &self.tasks
    }

    /// Returns tasks filtered by status
    pub fn filter_by_status(&self, status: TaskStatus) -> Vec<&Task> {
        self.tasks.iter()
            .filter(|task| task.status == status)
            .collect()
    }

    /// Returns pending tasks
    pub fn list_pending(&self) -> Vec<&Task> {
        self.filter_by_status(TaskStatus::Pending)
    }

    /// Returns in-progress tasks
    pub fn list_in_progress(&self) -> Vec<&Task> {
        self.filter_by_status(TaskStatus::InProgress)
    }

    /// Returns completed tasks
    pub fn list_completed(&self) -> Vec<&Task> {
        self.filter_by_status(TaskStatus::Completed)
    }

    /// Returns the number of tasks in the list
    pub fn count(&self) -> usize {
        self.tasks.len()
    }
}