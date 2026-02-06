use crate::Priority::*;
use clap::ValueEnum;
use comfy_table::{Cell, CellAlignment, Color, Table};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, ValueEnum)]
pub enum Priority {
    High,
    Medium,
    Low,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, ValueEnum)]
pub enum SortOpt {
    Id,
    Priority,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub completed: bool,
    pub priority: Priority,
}

pub fn edit_priority(todos: &mut Vec<Task>, id: &u32, priority: Priority) {
    match todos.iter_mut().find(|task| task.id == *id) {
        Some(task) => {
            task.priority = priority;
            println!("Task {} priority changed", id);
        }
        None => println!("Task {} not found", id),
    }
}

pub fn add_multiple_task(todos: &mut Vec<Task>, values: &Vec<String>) {
    for elt in values {
        let mut new_id: u32 = todos.len().try_into().unwrap();
        if new_id == 0 {
            new_id = 1;
        }
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
            priority: Priority::Low,
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
    table.set_header(vec!["ID", "Task", "Priority"]);
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
    // Column 2 is Priority -> Center it
    table
        .column_mut(2)
        .unwrap()
        .set_cell_alignment(CellAlignment::Center);

    for task in todos {
        // Create the ID cell (we can style this too if we want!)
        let id_cell = Cell::new(&task.id).fg(Color::White);
        // Create the Task cell with plain text first
        let mut task_cell = Cell::new(&task.description);
        // Apply color based on status
        if task.completed {
            task_cell = task_cell.fg(Color::Green);
        } else {
            task_cell = task_cell.fg(Color::Red);
        }
        let priority_cell = match task.priority {
            Priority::High => Cell::new("High").fg(Color::Red),
            Priority::Medium => Cell::new("Medium").fg(Color::Yellow),
            Priority::Low => Cell::new("Low").fg(Color::Green),
        };
        // Add the row using these smart cells
        table.add_row([id_cell, task_cell, priority_cell]);
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

pub fn clear_completed_tasks(todos: &mut Vec<Task>) {
    let initial_len = todos.len();
    todos.retain(|task| !task.completed);
    let removed_count = initial_len - todos.len();
    if removed_count == 0 {
        println!("No completed tasks to clear.");
    } else {
        println!("Cleared {} completed task(s).", removed_count);
    }
}

pub fn sort(todos: &mut Vec<Task>, option: &SortOpt) {
    match option {
        SortOpt::Priority => {
            let mut high = todos.clone();
            let mut med = todos.clone();
            let mut low = todos.clone();
            high.retain(|task| task.priority == High);
            med.retain(|task| task.priority == Medium);
            low.retain(|task| task.priority == Low);
            todos.retain(|_| false);
            for task in high {
                todos.push(task);
            }
            for task in med {
                todos.push(task);
            }
            for task in low {
                todos.push(task);
            }
        }
        SortOpt::Id => {
            let mut clone = todos.clone();
            todos.retain(|_| false);
            let mut i = 0;
            let len = clone.len();
            while i < len {
                todos.push(todos.iter().find(|task| task.id == i));
            }
        }
    }
}
