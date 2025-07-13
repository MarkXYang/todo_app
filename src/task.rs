use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufRead, Write};

/// Represents a task in the todo list
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: usize,
    pub description: String,
    pub done: bool,
}

impl Task {
    /// Creates a new task with the given description
    pub fn new(description: String) -> Self {
        Self {
            id: 0, // This will be set when added to a collection
            description,
            done: false,
        }
    }
}

/// Task management functions
pub struct TaskManager;

impl TaskManager {
    /// Adds a new task to the collection
    pub fn add_task(tasks: &mut Vec<Task>, description: String) {
        let new_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        let mut task = Task::new(description);
        task.id = new_id;
        tasks.push(task);
        println!("Added task {}.", new_id);
    }
    
    /// Lists all tasks in the collection
    pub fn list_tasks(tasks: &[Task]) {
        if tasks.is_empty() {
            println!("No tasks in the to-do list.");
        } else {
            for task in tasks {
                println!(
                    "[{}] {} - {}",
                    if task.done { "x" } else { " " },
                    task.id,
                    task.description
                );
            }
        }
    }
    
    /// Marks a task as completed
    pub fn complete_task(tasks: &mut [Task], id: usize) {
        if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
            task.done = true;
            println!("Completed task {}.", id);
        } else {
            println!("Task not found.");
        }
    }
    
    /// Removes a task from the collection
    pub fn remove_task(tasks: &mut Vec<Task>, id: usize) {
        if tasks.iter().any(|t| t.id == id) {
            tasks.retain(|t| t.id != id);
            println!("Removed task {}.", id);
        } else {
            println!("Task not found.");
        }
    }

    /// Saves tasks to a file
    pub fn save_tasks(tasks: &[Task], file_path: &str) -> Result<(), io::Error> {
        let mut file = File::create(file_path)?;
        for task in tasks {
            let serialized = serde_json::to_string(task)?;
            writeln!(file, "{}", serialized)?;
        }
        Ok(())
    }
    
    /// Loads tasks from a file
    pub fn load_tasks(file_path: &str) -> Result<Vec<Task>, io::Error> {
        let file = File::open(file_path)?;
        let reader = io::BufReader::new(file);
        let mut tasks = Vec::new();
        
        for (line_num, line) in reader.lines().enumerate() {
            let line = line?;
            if !line.trim().is_empty() {
                match serde_json::from_str::<Task>(&line) {
                    Ok(task) => tasks.push(task),
                    Err(e) => eprintln!("Warning: Invalid JSON on line {}: {}", line_num + 1, e),
                }
            }
        }
        
        Ok(tasks)
    }
}
