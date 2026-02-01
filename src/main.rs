use colored::*;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::io;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]

struct Task {
    id: u32,
    description: String,
    completed: bool,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // If len is 1 (just the program name), run interactive.
    // If len is 2+ (program + command), run CLI.
    if args.len() < 2 {
        let mut todos = load_todos();
        loop {
            println!(
                "|==> 1. Add |==> 2. View |==> 3. Complete |==> 4. Delete |==> 5. Save |==> 6. Quit"
            );
            let mut choice = String::new();
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line.");
            let choice: u32 = match choice.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            match choice {
                1 => add_task(&mut todos),
                2 => show_todo(&todos), // Reused your helper function here!
                3 => complete_task(&mut todos),
                4 => delete_task(&mut todos),
                5 => save_tasks(&todos),
                6 => {
                    save_tasks(&todos);
                    println!("Goodbye");
                    break;
                }
                _ => println!("Invalid Choice"),
            }
        }
    } else {
        // === CLI MODE ===
        let mut todos = load_todos();
        let command = &args[1];

        let values = args[2..].to_vec();

        match command.as_str() {
            "view" => show_todo(&todos),
            "add" => add_multiple_task(&mut todos, &values),
            "complete" => complete_multiple_task(&mut todos, &values),
            "delete" => delete_multiple_task(&mut todos, &values),
            _ => println!("Unknown command: {}.", command),
        }
        save_tasks(&todos);
    }
}

fn add_task(todos: &mut Vec<Task>) {
    println!("What is the task ?");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line.");
    let new_id: u32 = todos.len().try_into().unwrap();
    let new_task = Task {
        id: new_id + 1,
        description: name.trim().to_string(),
        completed: false,
    };
    todos.push(new_task);
    println!("Task Added !");
}

fn complete_task(todos: &mut Vec<Task>) {
    println!("What is the id to complete ?");
    let mut id_to_complete = String::new();
    io::stdin()
        .read_line(&mut id_to_complete)
        .expect("Failed to read line");
    let id_to_complete: u32 = match id_to_complete.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };
    match todos.iter_mut().find(|task| task.id == id_to_complete) {
        Some(task) => {
            task.completed = !task.completed;
            println!("Task {} completed", id_to_complete);
        }
        None => println!("Task {} not found", id_to_complete),
    }
}

fn delete_task(todos: &mut Vec<Task>) {
    println!("What is the id to delete ?");
    let mut id_to_delete = String::new();
    io::stdin()
        .read_line(&mut id_to_delete)
        .expect("Failed to read line");
    let id_to_delete: u32 = match id_to_delete.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };
    todos.retain(|task| task.id != id_to_delete);
    println!("Task succesfully deleted");
}

fn add_multiple_task(todos: &mut Vec<Task>, values: &Vec<String>) {
    for elt in values {
        let mut new_id: u32 = todos.len().try_into().unwrap();
        loop {
            match todos.iter_mut().find(|task| task.id == new_id) {
                Some(_) => new_id += 1,
                None => break,
            }
        }
        let new_task = Task {
            id: new_id,
            description: elt.trim().to_string(),
            completed: false,
        };
        todos.push(new_task);
    }
    println!("Tasks Added !");
}

fn complete_multiple_task(todos: &mut Vec<Task>, values: &Vec<String>) {
    for elt in values {
        let id_to_complete: u32 = match elt.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match todos.iter_mut().find(|task| task.id == id_to_complete) {
            Some(task) => {
                task.completed = !task.completed;
                println!("Task {} completed", id_to_complete);
            }
            None => println!("Task {} not found", id_to_complete),
        }
    }
}

fn delete_multiple_task(todos: &mut Vec<Task>, values: &Vec<String>) {
    for elt in values {
        let id_to_delete: u32 = match elt.trim().parse() {
            Ok(num) => num,
            Err(_) => return,
        };
        todos.retain(|task| task.id != id_to_delete);
    }
    println!("Tasks succesfully deleted");
}

fn save_tasks(todos: &Vec<Task>) {
    /*let mut content = String::new();
    for task in todos {
        content
            .push_str(format!("{}||{}||{}\n", task.id, task.description, task.completed).as_str());
    }*/
    let content = serde_json::to_string_pretty(&todos).expect("Failed to serialize.");
    std::fs::write(get_db_path(), content).expect("Failed to write file.");
    println!("Tasks saved");
}

fn load_todos() -> Vec<Task> {
    /*match std::fs::read_to_string(get_db_path()) {
        Ok(content) => {
            let mut todos = Vec::<Task>::new();
            for line in content.lines() {
                let parts: Vec<&str> = line.split("||").collect();
                if parts.len() == 3 {
                    let new_task = Task {
                        id: parts[0].trim().parse().unwrap(),
                        description: parts[1].trim().to_string(),
                        completed: parts[2].trim().parse().unwrap(),
                    };
                    todos.push(new_task);
                }
            }
            return todos;
        }
        Err(_) => {
            println!("Failed to load file, starting fresh.");
            return Vec::<Task>::new();
        }
    };*/
    match std::fs::read_to_string(get_db_path()) {
        Ok(content) => {
            // This one line parses the JSON string back into a Vec<Task>
            // If the file is corrupted, it returns an error, which we unwrap
            serde_json::from_str(&content).unwrap_or_else(|_| {
                println!("File corrupted, starting fresh.");
                Vec::new()
            })
        }
        Err(_) => Vec::new(),
    }
}

fn show_todo(todos: &Vec<Task>) {
    for task in todos {
        let color_desc = if task.completed {
            task.description.green()
        } else {
            task.description.red()
        };
        println!("> {}.{}", task.id, color_desc);
    }
}

fn get_db_path() -> PathBuf {
    // Try to find HOME (Linux/Mac), if not found, try USERPROFILE (Windows)
    let home_dir = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .expect("Could not find home directory.");

    let mut path = PathBuf::from(home_dir);
    path.push(".todo_list_data.csv"); // .push handles the slash automatically
    return path;
}
