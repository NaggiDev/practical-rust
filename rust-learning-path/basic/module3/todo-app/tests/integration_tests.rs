use todo_app::task::{Task, TaskStatus};
use todo_app::task_list::TaskList;

// Note: For these tests to work, we need to make the modules public in main.rs
// or create a lib.rs file that re-exports them

#[test]
fn test_add_task() {
    let mut task_list = TaskList::new();
    let task = task_list.add_task("Test Task".to_string(), None, None);
    
    assert_eq!(task.title, "Test Task");
    assert_eq!(task.status, TaskStatus::Pending);
    assert_eq!(task_list.count(), 1);
}

#[test]
fn test_get_task() {
    let mut task_list = TaskList::new();
    task_list.add_task("Task 1".to_string(), None, None);
    
    let task = task_list.get_task(1);
    assert!(task.is_some());
    assert_eq!(task.unwrap().title, "Task 1");
    
    let non_existent = task_list.get_task(999);
    assert!(non_existent.is_none());
}

#[test]
fn test_update_task_status() {
    let mut task_list = TaskList::new();
    task_list.add_task("Task 1".to_string(), None, None);
    
    let task = task_list.get_task_mut(1).unwrap();
    task.mark_in_progress();
    assert_eq!(task.status, TaskStatus::InProgress);
    
    task.mark_completed();
    assert_eq!(task.status, TaskStatus::Completed);
    
    task.mark_pending();
    assert_eq!(task.status, TaskStatus::Pending);
}

#[test]
fn test_remove_task() {
    let mut task_list = TaskList::new();
    task_list.add_task("Task 1".to_string(), None, None);
    task_list.add_task("Task 2".to_string(), None, None);
    
    assert_eq!(task_list.count(), 2);
    
    let result = task_list.remove_task(1);
    assert!(result);
    assert_eq!(task_list.count(), 1);
    
    let result = task_list.remove_task(999);
    assert!(!result);
    assert_eq!(task_list.count(), 1);
}

#[test]
fn test_filter_by_status() {
    let mut task_list = TaskList::new();
    task_list.add_task("Task 1".to_string(), None, None);
    task_list.add_task("Task 2".to_string(), None, None);
    task_list.add_task("Task 3".to_string(), None, None);
    
    // All tasks start as pending
    assert_eq!(task_list.list_pending().len(), 3);
    assert_eq!(task_list.list_in_progress().len(), 0);
    assert_eq!(task_list.list_completed().len(), 0);
    
    // Mark one task as in progress
    let task = task_list.get_task_mut(2).unwrap();
    task.mark_in_progress();
    
    assert_eq!(task_list.list_pending().len(), 2);
    assert_eq!(task_list.list_in_progress().len(), 1);
    
    // Mark one task as completed
    let task = task_list.get_task_mut(3).unwrap();
    task.mark_completed();
    
    assert_eq!(task_list.list_pending().len(), 1);
    assert_eq!(task_list.list_in_progress().len(), 1);
    assert_eq!(task_list.list_completed().len(), 1);
}