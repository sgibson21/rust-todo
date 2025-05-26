mod cli;
mod format;
mod storage;
mod task;

use cli::process_command;
use std::path::PathBuf;
use storage::{JSONStorage, Storage};

fn main() {
    let path = PathBuf::from("tasks.json");
    let storage = JSONStorage::new(path);

    let mut tasks = storage.load().unwrap_or_else(|err| {
        eprintln!("Error loading tasks {}", err);
        vec![]
    });

    process_command(&mut tasks);

    match storage.save(&tasks) {
        Ok(_) => {}
        Err(e) => println!("Error saving tasks: {}", e),
    }
}
