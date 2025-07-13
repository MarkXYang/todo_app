use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufRead, Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: usize,
    pub description: String,
    pub done: bool,
}

pub fn add_task(tasks: &mut Vec<Task>, description: String) {
    let new_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    tasks.push(Task {
        id: new_id,
        description,
        done: false,
    });
    println!("Added task {}.", new_id);
}

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

pub fn complete_task(tasks: &mut [Task], id: usize) {
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.done = true;
        println!("Completed task {}.", id);
    } else {
        println!("Task not found.");
    }
}

pub fn remove_task(tasks: &mut Vec<Task>, id: usize) {
    if tasks.iter().any(|t| t.id == id) {
        tasks.retain(|t| t.id != id);
        println!("Removed task {}.", id);
    } else {
        println!("Task not found.");
    }
}

pub fn load_tasks() -> Result<Vec<Task>, io::Error> {
    let file = File::open("todo.txt")?;
    let reader = io::BufReader::new(file);
    let tasks = reader
        .lines()
        .filter_map(Result::ok)
        .filter_map(|line| serde_json::from_str(&line).ok())
        .collect();
    Ok(tasks)
}

pub fn save_tasks(tasks: &[Task]) -> Result<(), io::Error> {
    let mut file = File::create("todo.txt")?;
    for task in tasks {
        let serialized = serde_json::to_string(task)?;
        writeln!(file, "{}", serialized)?;
    }
    Ok(())
} 