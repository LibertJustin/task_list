use clap::{CommandFactory, Parser, Subcommand};

use std::process::Command;
use std::str;
mod models;
use models::*;
mod storage;
use storage::*;

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
    /// Removes all completed tasks
    Clear,
    /// Change the priority of the task with the id provided as: priority id new_priority
    Priority { id: u32, priority: Priority },
    /// Sort by the option passed as : sort sort_option
    Sort { opt: SortOpt },
    #[command(hide = true)]
    Completion { shell: clap_complete::Shell },
}
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
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
            let _ = Command::new("sh")
                .arg("-c")
                .arg("clear")
                .output()
                .expect("failed to execute process");
            show_todo(&todos);
        }
        Commands::Edit { id, task } => {
            edit_task(&mut todos, &id, task);
        }
        Commands::Clear => {
            clear_completed_tasks(&mut todos);
        }
        Commands::Priority { id, priority } => {
            edit_priority(&mut todos, &id, priority);
        }
        Commands::Sort { opt } => {
            sort(&mut todos, &opt);
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
    //sort(&mut todos, &SortOpt::Id);
    save_tasks(&todos);
}
