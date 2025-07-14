use std::env;
mod task;
use task::{add_task, list_tasks, complete_task, remove_task, load_tasks, save_tasks};

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
            "help" => {
                println!("Usage: todo <command> [options]");
                println!("Commands:");
                println!("  add <task description> - Add a new task");
                println!("  list - List all tasks");
                println!("  done <task ID> - Mark a task as done");
                println!("  remove <task ID> - Remove a task");
                println!("  help - Show this help message");
            }
            _ => println!("Unknown command. Use 'help' for usage information."),
        }
    } else {
        list_tasks(&tasks);
    }

    // Save tasks (no-op for database, but keeping for compatibility)
    if let Err(e) = save_tasks(&tasks) {
        eprintln!("Warning: Failed to save tasks: {}", e);
    }
}