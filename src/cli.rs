use std::io;

use crate::format::Format;
use crate::task::Task;
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::generate;

#[derive(Parser)]
#[command(name = "todo_app")] // name of binary to run in shell
#[command(about = "A simple CLI Todo application", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new task
    Add { description: String },
    /// List all commands
    List {
        /// Output format: plain or JSON
        #[arg(long, default_value = "plain")]
        format: Format,
    },
    /// Mark a task as complete
    Complete { id: u32 },
    /// Remove a task
    Remove { id: u32 },
    /// Generate shell completions
    Completions {
        /// Shell type: bash, zsh, fish, powershell, elvish
        #[arg(long)]
        shell: clap_complete::Shell,
    },
}

pub fn process_command(tasks: &mut Vec<Task>) {
    let cli = Cli::parse();
    let command = cli.command;

    match command {
        Commands::Add { description } => {
            let id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
            let task = Task {
                id,
                description,
                completed: false,
            };

            tasks.push(task);
        }
        Commands::List { format } => {
            println!("{}", format.format_tasks(tasks));
        }
        Commands::Complete { id } => {
            if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
                task.completed = true;
            }
        }
        Commands::Remove { id } => {
            if let Some((index, _)) = tasks.iter().enumerate().find(|(_, task)| task.id == id) {
                tasks.remove(index);
            }
        }
        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            let name = cmd.get_name().to_string();
            generate(shell, &mut cmd, name, &mut io::stdout());

            // let out_dir = env::var_os("OUT_DIR").unwrap();
            // let mut cmd = todo_app::Cli::command();
            // generate_to(Bash, &mut cmd, "todo_app", Path::new(&out_dir)).unwrap();
        }
    }
}
