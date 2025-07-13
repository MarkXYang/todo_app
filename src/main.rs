use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: usize,
    description: String,
    done: bool,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut tasks = load_tasks().unwrap_or_else(|_| Vec::new());

    if args.len() > 1 {
        let command = &args[1];
        match command.as_str() {
            "add" => {
                if args.len() > 2 {
                    add_task(&mut tasks, args[2..].join(" "));
                } else {
                    println!("Usage: add <task description>");
                }
            }
            "list" => list_tasks(&tasks),
            "done" => {
                if args.len() > 2 {
                    if let Ok(id) = args[2].parse::<usize>() {
                        complete_task(&mut tasks, id);
                    } else {
                        println!("Invalid task ID.");
                    }
                } else {
                    println!("Usage: done <task ID>");
                }
            }
            "remove" => {
                if args.len() > 2 {
                    if let Ok(id) = args[2].parse::<usize>() {
                        remove_task(&mut tasks, id);
                    } else {
                        println!("Invalid task ID.");
                    }
                } else {
                    println!("Usage: remove <task ID>");
                }
            }
            _ => println!("Unknown command."),
        }
    } else {
        list_tasks(&tasks);
    }

    save_tasks(&tasks).expect("Failed to save tasks.");
}

fn add_task(tasks: &mut Vec<Task>, description: String) {
    let new_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    tasks.push(Task {
        id: new_id,
        description,
        done: false,
    });
    println!("Added task {}.", new_id);
}

fn list_tasks(tasks: &[Task]) {
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

fn complete_task(tasks: &mut [Task], id: usize) {
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.done = true;
        println!("Completed task {}.", id);
    } else {
        println!("Task not found.");
    }
}

fn remove_task(tasks: &mut Vec<Task>, id: usize) {
    if tasks.iter().any(|t| t.id == id) {
        tasks.retain(|t| t.id != id);
        println!("Removed task {}.", id);
    } else {
        println!("Task not found.");
    }
}

fn load_tasks() -> Result<Vec<Task>, io::Error> {
    let file = File::open("todo.txt")?;
    let reader = io::BufReader::new(file);
    let tasks = reader
        .lines()
        .filter_map(Result::ok)
        .filter_map(|line| serde_json::from_str(&line).ok())
        .collect();
    Ok(tasks)
}

fn save_tasks(tasks: &[Task]) -> Result<(), io::Error> {
    let mut file = File::create("todo.txt")?;
    for task in tasks {
        let serialized = serde_json::to_string(task)?;
        writeln!(file, "{}", serialized)?;
    }
    Ok(())
}