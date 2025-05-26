use crate::task::Task;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

pub trait Storage {
    fn load(&self) -> Result<Vec<Task>, Box<dyn Error>>;
    fn save(&self, tasks: &[Task]) -> Result<(), Box<dyn Error>>;
}

pub struct JSONStorage {
    pub path: PathBuf,
}

impl JSONStorage {
    pub fn new(path: PathBuf) -> JSONStorage {
        JSONStorage { path }
    }
}

impl Storage for JSONStorage {
    fn load(&self) -> Result<Vec<Task>, Box<dyn Error>> {
        if !self.path.exists() {
            return Ok(vec![]); // no Tasks for no path
        }

        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        let tasks = serde_json::from_reader(reader)?;

        Ok(tasks)
    }

    fn save(&self, tasks: &[Task]) -> Result<(), Box<dyn Error>> {
        if !self.path.exists() || tasks.is_empty() {
            return Ok(()); // nothing to do if no tasks
        }

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, tasks)?;

        Ok(())
    }
}
