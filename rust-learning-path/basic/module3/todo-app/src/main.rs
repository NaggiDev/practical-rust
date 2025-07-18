mod task;
mod task_list;

use std::io::{self, Write};
use chrono::Local;
use task::{Task, TaskStatus};
use task_list::TaskList;

fn main() {
    println!("Welcome to the Rust To-Do List Application!");
    println!("Type 'help' to see available commands.");

    let mut task_list = TaskList::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let input = input.trim();
        
        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        let command = parts[0].to_lowercase();
        let args = parts.get(1).map(|s| s.to_string());

        match command.as_str() {
            "add" => {
                if let Some(title) = args {
                    let task = task_list.add_task(title, None, None);
                    println!("Task added: {}", task);
                } else {
                    println!("Error: Task title required");
                }
            },
            "list" => {
                let tasks = task_list.list_all();
                if tasks.is_empty() {
                    println!("No tasks found.");
                } else {
                    println!("Tasks:");
                    for task in tasks {
                        println!("{}", task);
                    }
                }
            },
            "pending" => {
                let tasks = task_list.list_pending();
                if tasks.is_empty() {
                    println!("No pending tasks found.");
                } else {
                    println!("Pending Tasks:");
                    for task in tasks {
                        println!("{}", task);
                    }
                }
            },
            "inprogress" => {
                let tasks = task_list.list_in_progress();
                if tasks.is_empty() {
                    println!("No in-progress tasks found.");
                } else {
                    println!("In-Progress Tasks:");
                    for task in tasks {
                        println!("{}", task);
                    }
                }
            },
            "completed" => {
                let tasks = task_list.list_completed();
                if tasks.is_empty() {
                    println!("No completed tasks found.");
                } else {
                    println!("Completed Tasks:");
                    for task in tasks {
                        println!("{}", task);
                    }
                }
            },
            "start" => {
                if let Some(id_str) = args {
                    if let Ok(id) = id_str.parse::<usize>() {
                        if let Some(task) = task_list.get_task_mut(id) {
                            task.mark_in_progress();
                            println!("Task #{} marked as in progress", id);
                        } else {
                            println!("Error: Task not found");
                        }
                    } else {
                        println!("Error: Invalid task ID");
                    }
                } else {
                    println!("Error: Task ID required");
                }
            },
            "complete" => {
                if let Some(id_str) = args {
                    if let Ok(id) = id_str.parse::<usize>() {
                        if let Some(task) = task_list.get_task_mut(id) {
                            task.mark_completed();
                            println!("Task #{} marked as completed", id);
                        } else {
                            println!("Error: Task not found");
                        }
                    } else {
                        println!("Error: Invalid task ID");
                    }
                } else {
                    println!("Error: Task ID required");
                }
            },
            "remove" => {
                if let Some(id_str) = args {
                    if let Ok(id) = id_str.parse::<usize>() {
                        if task_list.remove_task(id) {
                            println!("Task #{} removed", id);
                        } else {
                            println!("Error: Task not found");
                        }
                    } else {
                        println!("Error: Invalid task ID");
                    }
                } else {
                    println!("Error: Task ID required");
                }
            },
            "help" => {
                println!("Available commands:");
                println!("  add <title>       - Add a new task");
                println!("  list              - List all tasks");
                println!("  pending           - List pending tasks");
                println!("  inprogress        - List in-progress tasks");
                println!("  completed         - List completed tasks");
                println!("  start <id>        - Mark a task as in progress");
                println!("  complete <id>     - Mark a task as completed");
                println!("  remove <id>       - Remove a task");
                println!("  help              - Show this help message");
                println!("  exit              - Exit the application");
            },
            "exit" => {
                println!("Goodbye!");
                break;
            },
            _ => {
                println!("Unknown command. Type 'help' to see available commands.");
            }
        }
    }
}