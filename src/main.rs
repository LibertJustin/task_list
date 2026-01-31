use std::convert::TryInto;
use std::io;

struct Task {
    id: u32,
    description: String,
    completed: bool,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        let mut todos = load_todos();
        println!("=> 1.Add\n=> 2.View\n=> 3.Complete\n=> 4.Delete\n=> 5.Save\n=> 6.Quit");
        loop {
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
                        println!(
                            "[{}]-> {}.{};",
                            if task.completed { "*" } else { " " },
                            task.id,
                            task.description
                        );
                    }
                }
                3 => complete_task(&mut todos),
                6 => {
                    save_tasks(&todos);
                    println!("Goodbye");
                    break;
                }
                5 => save_tasks(&todos),
                4 => delete_task(&mut todos),
                _ => println!("Invalid Choice"),
            }
        }
    } else {
        let mut todos = load_todos();
        let command = &args[1];

        let mut i: usize = 2;
        let mut values = Vec::<String>::new();

        while i < args.len() {
            let val = args[i].clone();
            values.push(val);
            i += 1;
        }

        match command.as_str() {
            "view" => show_todo(&todos),
            "add" => add_multiple_task(&mut todos, &values),
            "complete" => complete_multiple_task(&mut todos, &values),
            "delete" => delete_multiple_task(&mut todos, &values),
            _ => println!("Unknown command : {}.", command),
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
}

fn save_tasks(todos: &Vec<Task>) {
    let mut content = String::new();
    for task in todos {
        content
            .push_str(format!("{}||{}||{}\n", task.id, task.description, task.completed).as_str());
    }
    std::fs::write("todos.csv", content).expect("Failed to write file.");
}

fn load_todos() -> Vec<Task> {
    match std::fs::read_to_string("todos.csv") {
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
    };
}

fn show_todo(todos: &Vec<Task>) {
    for task in todos {
        println!(
            "[{}]-> {}.{};",
            if task.completed { "*" } else { " " },
            task.id,
            task.description
        );
    }
}
