use std::io;

struct Task {
    id: u32,
    description: String,
    completed: bool,
}

fn main() {
    let mut todos = Vec::<Task>::new();
    loop {
        println!("==> 1. Add\n==> 2.View\n==> 3.Complete\n==> 4.Quit");
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
            2 => {
                for task in &todos {
                    println!("[{}]->{}.{};", task.completed, task.id, task.description);
                }
            }
            3 => complete_task(&mut todos),
            4 => {
                println!("Goodbye");
                break;
            }
            _ => println!("Invalid Choice"),
        }
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
            task.completed = true;
            println!("Task {} completed", id_to_complete);
        }
        None => println!("Task {} not found", id_to_complete),
    }
}
