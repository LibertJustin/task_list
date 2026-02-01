use comfy_table::{Cell, CellAlignment, Color, Table};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub completed: bool,
}

pub fn add_multiple_task(todos: &mut Vec<Task>, values: &Vec<String>) {
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

pub fn complete_multiple_task(todos: &mut Vec<Task>, values: &Vec<u32>) {
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

pub fn delete_multiple_task(todos: &mut Vec<Task>, values: &Vec<u32>) {
    for id in values {
        todos.retain(|task| task.id != *id);
    }
    println!("Tasks succesfully deleted");
}

pub fn show_todo(todos: &Vec<Task>) {
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

pub fn edit_task(todos: &mut Vec<Task>, id: &u32, new_task: String) {
    match todos.iter_mut().find(|task| task.id == *id) {
        Some(task) => {
            task.description = new_task;
            println!("Task {} changed", id);
        }
        None => println!("Task {} not found", id),
    }
}
