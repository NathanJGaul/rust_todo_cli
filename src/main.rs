use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Write};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: u32,
    description: String,
    complete: bool,
}

impl Task {
    fn new(id: u32, description: String) -> Task {
        Task {
            id,
            description,
            complete: false,
        }
    }
}

struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    fn new() -> TaskManager {
        TaskManager { tasks: Vec::new() }
    }

    fn add_task(&mut self, description: String) {
        let id = self.tasks.len() as u32 + 1;
        let task = Task::new(id, description);
        self.tasks.push(task);
    }

    fn remove_task(&mut self, id: u32) {
        self.tasks.retain(|task| task.id != id);
    }

    fn list_tasks(&self) {
        for task in &self.tasks {
            println!("{:?}", task);
        }
    }

    fn save_tasks(&self) -> io::Result<()> {
        let file = File::create("tasks.json")?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &self.tasks)?;
        Ok(())
    }

    fn load_tasks(&mut self) -> io::Result<()> {
        let file = File::open("tasks.json")?;
        let reader = BufReader::new(file);
        self.tasks = serde_json::from_reader(reader)?;
        Ok(())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut manager = TaskManager::new();

    if let Err(e) = manager.load_tasks() {
        println!("Failed to load tasks: {}", e);
    }

    if args.len() < 2 {
        eprintln!("Usage: todo <command> [arguments]");
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                eprintln!("Usage: todo add <task_description>");
            } else {
                let description = args[2..].join(" ");
                manager.add_task(description);
                if let Err(e) = manager.save_tasks() {
                    eprintln!("Failed to save tasks: {}", e);
                }
            }
        }
        "remove" => {
            if args.len() < 3 {
                eprintln!("Usage: todo remove <task_id>");
            } else {
                let id: u32 = args[2].parse().expect("Please provide a valid task ID");
                manager.remove_task(id);
                if let Err(e) = manager.save_tasks() {
                    eprintln!("Failed to save tasks: {}", e);
                }
            }
        }
        "list" => manager.list_tasks(),
        _ => eprintln!("Unkown comman: {}", command),
    }
}
