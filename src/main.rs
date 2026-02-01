use clap::{CommandFactory, Parser, Subcommand};
//use clap_complete::*;
//use colored::*;
use comfy_table::{Cell, CellAlignment, Color, Table};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::path::PathBuf;

#[derive(Subcommand)]
enum Commands {
    /// Add new tasks to your list: add "task1" "task2" ...
    Add { task: Vec<String> },
    /// Lists your current tasks
    View,
    /// Delete tasks from your list: delete id1 id2 ...
    Delete { id: Vec<u32> },
    /// Complete/Uncomplete tasks from your list: complete id1 id2 ...
    Complete { id: Vec<u32> },
    /// Edit the task description of the task with the id provided as: edit id new_description
    Edit { id: u32, task: String },
    #[command(hide = true)]
    Completion { shell: clap_complete::Shell },
}
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    description: String,
    completed: bool,
}

fn main() {
    let args = Cli::parse();
    // === CLI MODE ===
    let mut todos = load_todos();

    match args.command {
        Commands::Add { task } => {
            add_multiple_task(&mut todos, &task);
        }
        Commands::Complete { id } => {
            complete_multiple_task(&mut todos, &id);
        }
        Commands::Delete { id } => {
            delete_multiple_task(&mut todos, &id);
        }
        Commands::View => {
            show_todo(&todos);
        }
        Commands::Edit { id, task } => {
            edit_task(&mut todos, &id, task);
        }
        Commands::Completion { shell } => {
            let cmd = Cli::command();
            // 1. Filter out the "completion" subcommand
            // We clone the others so we have a clean list
            let subcommands = cmd
                .get_subcommands()
                .filter(|sc| sc.get_name() != "completion")
                .cloned()
                .collect::<Vec<_>>();
            // 2. Create a fresh "shadow" command for generation
            // We add the filtered subcommands back to it
            let mut shadow_cmd = clap::Command::new("todo").subcommands(subcommands);
            // 3. Generate the script using the shadow command
            clap_complete::generate(shell, &mut shadow_cmd, "todo", &mut std::io::stdout());
        }
    }

    save_tasks(&todos);
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

fn complete_multiple_task(todos: &mut Vec<Task>, values: &Vec<u32>) {
    for id in values {
        match todos.iter_mut().find(|task| task.id == *id) {
            Some(task) => {
                task.completed = !task.completed;
                println!("Task {} completed", id);
            }
            None => println!("Task {} not found", id),
        }
    }
}

fn delete_multiple_task(todos: &mut Vec<Task>, values: &Vec<u32>) {
    for id in values {
        todos.retain(|task| task.id != *id);
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
    // Inside the View match arm:
    let mut table = Table::new();
    table.set_header(vec!["ID", "Task"]);
    // Column 0 is ID -> Center it
    table
        .column_mut(0)
        .unwrap()
        .set_cell_alignment(CellAlignment::Center);
    // Column 1 is Task -> Left align (default, but good to be explicit)
    table
        .column_mut(1)
        .unwrap()
        .set_cell_alignment(CellAlignment::Left);

    for task in todos {
        // Create the ID cell (we can style this too if we want!)
        let id_cell = Cell::new(&task.id).fg(Color::White);
        // Create the Task cell with plain text first
        let mut task_cell = Cell::new(&task.description);
        // Apply color based on status
        if task.completed {
            task_cell = task_cell.fg(Color::DarkGreen);
        } else {
            task_cell = task_cell.fg(Color::DarkRed);
        }
        // Add the row using these smart cells
        table.add_row([id_cell, task_cell]);
    }
    println!("{table}");
}

fn edit_task(todos: &mut Vec<Task>, id: &u32, new_task: String) {
    match todos.iter_mut().find(|task| task.id == *id) {
        Some(task) => {
            task.description = new_task;
            println!("Task {} changed", id);
        }
        None => println!("Task {} not found", id),
    }
}

fn get_db_path() -> PathBuf {
    // Try to find HOME (Linux/Mac), if not found, try USERPROFILE (Windows)
    let home_dir = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .expect("Could not find home directory.");

    let mut path = PathBuf::from(home_dir);
    path.push(".todo_list_data.json"); // .push handles the slash automatically
    return path;
}
