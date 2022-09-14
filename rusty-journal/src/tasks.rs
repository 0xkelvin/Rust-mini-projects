use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::Deserialize;
use serde::Serialize;
use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::io::{Error, ErrorKind, Result, Seek, SeekFrom};
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Task {
        let created_at: DateTime<Utc> = Utc::now();
        Task { text, created_at }
    }
}

pub fn add_task(journal_file: PathBuf, task: Task) -> Result<()> {
    // Open the file
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_file)?;

    // consume the file's contents as a vector of tasks
    // let mut tasks: Vec<Task> = match serde_json::from_reader((&file)) {
    //     Ok(tasks) => tasks,
    //     Err(e) if e.is_eof() => Vec::new(),
    //     Err(e) => Err(e)?,
    // };

    // // rewind the file after reading from it
    // file.seek(SeekFrom::Start(0))?;

    let mut tasks = collect_tasks(&file)?;

    // write the modified tasks list back into the file
    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;
    Ok(())
}

pub fn complete_task(journal_file: PathBuf, task_position: usize) -> Result<()> {
    // Open the file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_file)?;
    
    // Consume the file's contents as a vector of tasks
    // let tasks = match serde_json::from_reader(file) {
    //     Ok(tasks) => tasks,
    //     Err(e) if e.is_eof() => Vec::new(),
    //     Err(e) => Err(e)?,
    // };
    let mut tasks = collect_tasks(&file)?;
    // remove the task
    if task_position == 0 || task_position > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid task ID"));

    }
    tasks.remove(task_position -1);

    file.set_len(0)?;

    // Write the modified task list back into the file
    serde_json::to_writer(file, &tasks)?;
    Ok(())
}

fn collect_tasks(mut file: &File) -> Result<Vec<Task>> {
    file.seek(SeekFrom::Start(0))?;
    let tasks = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    file.seek(SeekFrom::Start(0))?; // rewind the file after
    Ok(tasks)
}

pub fn list_tasks(journal_path: PathBuf) -> Result<()> {
    // Open the file
    let file = OpenOptions::new().read(true).open(journal_path)?;
    // Parse the file and collect the tasks
    let tasks = collect_tasks(&file)?;

    // Enumerate and display tasks, if any
    if tasks.is_empty() {
        println!("task list is empty");
    } else {
        let mut order: u32 = 1;
        for task in tasks {
            println!("{}: {}", order, task);
            order += 1;
        }
    }

    Ok(())
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<50} [{}]", self.text, created_at)
        }
}