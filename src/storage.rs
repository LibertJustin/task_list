use crate::models::Task; // Import Task from models
use std::path::PathBuf;

fn get_db_path() -> PathBuf {
    // Try to find HOME (Linux/Mac), if not found, try USERPROFILE (Windows)
    let home_dir = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .expect("Could not find home directory.");

    let mut path = PathBuf::from(home_dir);
    path.push(".todo_list_data.json"); // .push handles the slash automatically
    return path;
}

fn get_db_path_backup() -> PathBuf {
    // Try to find HOME (Linux/Mac), if not found, try USERPROFILE (Windows)
    let home_dir = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .expect("Could not find home directory.");

    let mut path = PathBuf::from(home_dir);
    path.push(".todo_list_data_backup.json"); // .push handles the slash automatically
    return path;
}

pub fn save_tasks(todos: &Vec<Task>) {
    let content = serde_json::to_string_pretty(&todos).expect("Failed to serialize.");
    std::fs::write(get_db_path(), &content).expect("Failed to write file.");
    std::fs::write(get_db_path_backup(), &content).expect("Failed to write file.");
}

pub fn load_todos() -> Vec<Task> {
    match std::fs::read_to_string(get_db_path()) {
        Ok(content) => {
            // This one line parses the JSON string back into a Vec<Task>
            // If the file is corrupted, it returns an error, which we unwrap
            serde_json::from_str(&content).unwrap_or_else(|_| {
                println!("File corrupted, trying backup.");
                load_todos_backup()
            })
        }
        Err(_) => Vec::new(),
    }
}

pub fn load_todos_backup() -> Vec<Task> {
    match std::fs::read_to_string(get_db_path_backup()) {
        Ok(content) => {
            // This one line parses the JSON string back into a Vec<Task>
            // If the file is corrupted, it returns an error, which we unwrap
            serde_json::from_str(&content).unwrap_or_else(|_| {
                println!("Backup corrupted, starting fresh.");
                Vec::new()
            })
        }
        Err(_) => Vec::new(),
    }
}
