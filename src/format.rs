use crate::task::Task;
use clap::ValueEnum;

pub trait TaskFormatter {
    fn format_tasks(&self, tasks: &[Task]) -> String;
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Format {
    Plain,
    Json,
}

impl Format {
    pub fn format_tasks(&self, tasks: &[Task]) -> String {
        match self {
            Self::Plain => tasks
                .iter()
                .map(|t| {
                    format!(
                        "[{}] {} - {}",
                        t.id,
                        if t.completed { "X" } else { " " },
                        t.description,
                    )
                })
                .collect::<Vec<String>>()
                .join("\n"),
            Self::Json => serde_json::to_string_pretty(tasks)
                .unwrap_or_else(|_| "Failed to serialise".to_string()),
        }
    }
}
