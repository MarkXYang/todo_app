use serde::{Deserialize, Serialize};
use rusqlite::{Connection, Result as SqliteResult, params};
use chrono::{DateTime, Utc};

/// Represents a task in the todo list
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: Option<usize>,
    pub description: String,
    pub done: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Task {
    /// Creates a new task with the given description
    pub fn new(description: String) -> Self {
        let now = Utc::now();
        Self {
            id: None,
            description,
            done: false,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Task management functions using SQLite database
pub struct TaskManager {
    conn: Connection,
}

impl TaskManager {
    /// Creates a new TaskManager with database connection
    pub fn new(db_path: &str) -> SqliteResult<Self> {
        let conn = Connection::open(db_path)?;
        let manager = TaskManager { conn };
        manager.init_database()?;
        Ok(manager)
    }

    /// Initializes the database with the tasks table
    fn init_database(&self) -> SqliteResult<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                description TEXT NOT NULL,
                done BOOLEAN NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    /// Adds a new task to the database
    pub fn add_task(&self, description: String) -> SqliteResult<usize> {
        let task = Task::new(description);
        let id = self.conn.execute(
            "INSERT INTO tasks (description, done, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
            params![
                task.description,
                task.done,
                task.created_at.to_rfc3339(),
                task.updated_at.to_rfc3339()
            ],
        )?;
        println!("Added task {}.", id);
        Ok(id as usize)
    }
    
    /// Lists all tasks from the database
    pub fn list_tasks(&self) -> SqliteResult<()> {
        let mut stmt = self.conn.prepare(
            "SELECT id, description, done, created_at, updated_at FROM tasks ORDER BY id"
        )?;
        
        let task_iter = stmt.query_map([], |row| {
            let created_at: String = row.get(3)?;
            let updated_at: String = row.get(4)?;
            Ok(Task {
                id: Some(row.get(0)?),
                description: row.get(1)?,
                done: row.get(2)?,
                created_at: DateTime::parse_from_rfc3339(&created_at)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                updated_at: DateTime::parse_from_rfc3339(&updated_at)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
            })
        })?;

        let tasks: Vec<Task> = task_iter.filter_map(|r| r.ok()).collect();
        
        if tasks.is_empty() {
            println!("No tasks in the to-do list.");
        } else {
            for task in tasks {
                println!(
                    "[{}] {} - {} (Created: {}, Updated: {})",
                    if task.done { "x" } else { " " },
                    task.id.unwrap_or(0),
                    task.description,
                    task.created_at.format("%Y-%m-%d %H:%M:%S"),
                    task.updated_at.format("%Y-%m-%d %H:%M:%S")
                );
            }
        }
        Ok(())
    }
    
    /// Marks a task as completed
    pub fn complete_task(&self, id: usize) -> SqliteResult<()> {
        let now = Utc::now();
        let rows_affected = self.conn.execute(
            "UPDATE tasks SET done = 1, updated_at = ?1 WHERE id = ?2",
            params![now.to_rfc3339(), id],
        )?;
        
        if rows_affected > 0 {
            println!("Completed task {}.", id);
        } else {
            println!("Task not found.");
        }
        Ok(())
    }
    
    /// Removes a task from the database
    pub fn remove_task(&self, id: usize) -> SqliteResult<()> {
        let rows_affected = self.conn.execute(
            "DELETE FROM tasks WHERE id = ?1",
            params![id],
        )?;
        
        if rows_affected > 0 {
            println!("Removed task {}.", id);
        } else {
            println!("Task not found.");
        }
        Ok(())
    }

    /// Gets all tasks as a vector
    pub fn get_all_tasks(&self) -> SqliteResult<Vec<Task>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, description, done, created_at, updated_at FROM tasks ORDER BY id"
        )?;
        
        let task_iter = stmt.query_map([], |row| {
            let created_at: String = row.get(3)?;
            let updated_at: String = row.get(4)?;
            Ok(Task {
                id: Some(row.get(0)?),
                description: row.get(1)?,
                done: row.get(2)?,
                created_at: DateTime::parse_from_rfc3339(&created_at)
                    .map(|dt| dt.with_timezone(&Utc))  // Fixed timezone handling
                    .unwrap_or_else(|_| Utc::now()),
                updated_at: DateTime::parse_from_rfc3339(&updated_at)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
            })
        })?;

        let tasks: Vec<Task> = task_iter.filter_map(|r| r.ok()).collect();
        Ok(tasks)
    }
}

// Backward compatibility functions for main.rs
pub fn add_task(tasks: &mut Vec<Task>, description: String) {
    if let Ok(manager) = TaskManager::new("todo.db") {
        if let Ok(_) = manager.add_task(description) {
            // Refresh the tasks vector
            if let Ok(new_tasks) = manager.get_all_tasks() {
                *tasks = new_tasks;
            }
        }
    }
}

pub fn list_tasks(_tasks: &[Task]) {
    if let Ok(manager) = TaskManager::new("todo.db") {
        let _ = manager.list_tasks();
    }
}

pub fn complete_task(_tasks: &mut [Task], id: usize) {
    if let Ok(manager) = TaskManager::new("todo.db") {
        let _ = manager.complete_task(id);
    }
}

pub fn remove_task(tasks: &mut Vec<Task>, id: usize) {
    if let Ok(manager) = TaskManager::new("todo.db") {
        if let Ok(_) = manager.remove_task(id) {
            // Refresh the tasks vector
            if let Ok(new_tasks) = manager.get_all_tasks() {
                *tasks = new_tasks;
            }
        }
    }
}

pub fn save_tasks(_tasks: &[Task]) -> Result<(), Box<dyn std::error::Error>> {
    // No-op for database storage - data is automatically saved
    Ok(())
}

pub fn load_tasks() -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    match TaskManager::new("todo.db") {
        Ok(manager) => Ok(manager.get_all_tasks()?),
        Err(e) => Err(Box::new(e)),
    }
}
