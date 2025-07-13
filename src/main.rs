use std::env;
mod task;
use task::{Task, TaskManager};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut tasks = TaskManager::load_tasks("todo.txt").unwrap_or_else(|_| Vec::new());

    if args.len() > 1 {
        let command = &args[1];
        match command.as_str() {
            "add" => {
                if args.len() > 2 {
                    TaskManager::add_task(&mut tasks, args[2..].join(" "));
                } else {
                    println!("Usage: add <task description>");
                }
            }
            "list" => TaskManager::list_tasks(&tasks),
            "done" => {
                if args.len() > 2 {
                    if let Ok(id) = args[2].parse::<usize>() {
                        TaskManager::complete_task(&mut tasks, id);
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
                        TaskManager::remove_task(&mut tasks, id);
                    } else {
                        println!("Invalid task ID.");
                    }
                } else {
                    println!("Usage: remove <task ID>");
                }
            }
            "help" => {
                println!("Usage: todo <command> [options]");
                println!("Commands:");
                println!("  add <task description> - Add a new task");
                println!("  list - List all tasks");
                println!("  done <task ID> - Mark a task as done");
            }
            _ => println!("Unknown command."),
        }
    } else {
        TaskManager::list_tasks(&tasks);
    }

    TaskManager::save_tasks(&tasks, "todo.txt").expect("Failed to save tasks.");
}