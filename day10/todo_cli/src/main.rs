use std::io::{self, Write};
use std::fs::{self, File};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    description: String,
    completed: bool,
}



fn main() {
    let mut tasks: Vec<Task> = load_tasks();
    loop {
        println!("\nTo-Do List Menu:");
        println!("1. Add Task");
        println!("2. View Tasks");
        println!("3. Mark as Complete");
        println!("4. Delete Task");
        println!("5. Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let choice = get_input("Enter your choice: ");
        match choice.trim() {
            "1" => add_task(&mut tasks),
            "2" => view_tasks(&tasks),
            "3" => mark_complete(&mut tasks),
            "4" => delete_task(&mut tasks),
            "5" => {
                save_tasks(&tasks);
                println!("✅ Task saved. Goodbye!");
                break;
            },
            _ => println!("❌ Invalid choice, please try again."),
        }
    }
}

// get user input from cli
fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}

// Load tasks from a file
fn load_tasks() -> Vec<Task> {
    match fs::read_to_string("tasks.json") {
        Ok(data) => serde_json::from_str(&data).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(),
    }
}

// Save tasks to a file
fn save_tasks(tasks: &Vec<Task>) {
    let json = serde_json::to_string_pretty(tasks).expect("Failed to serialize tasks");
    let mut file = File::create("tasks.json").expect("Failed to create file");
    file.write_all(json.as_bytes()).expect("Failed to write to file");
}

// Add a new task
fn add_task(tasks: &mut Vec<Task>) {
    let description = get_input("Enter task description: ");
    let id = tasks.len() as u32 + 1; // Simple ID generation
    let task = Task {
        id,
        description: description.trim().to_string(),
        completed: false,
    };
    tasks.push(task);
    println!("✅ Task added successfully.");
}

//view all tasks
fn view_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("❌ No tasks available.");
    } else {
        for task in tasks {
            let status = if task.completed { "✅" } else { "❌" };
            println!("{} [{}] {}", task.id, status, task.description);
        }
    }
}

// Mark a task as complete
fn mark_complete(tasks: &mut Vec<Task>) {
    let id_str = get_input("Enter task ID to mark as complete: ");
    if let Ok(id) = id_str.trim().parse::<u32>() {
        if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            println!("✅ Task marked as complete.");
        } else {
            println!("❌ Task with ID {} not found.", id);
        }
    } else {
        println!("❌ Invalid ID format.");
    }
}

// Delete a task
fn delete_task(tasks: &mut Vec<Task>) {
    let id_str = get_input("Enter task ID to delete: ");
    if let Ok(id) = id_str.trim().parse::<u32>() {
        if let Some(index) = tasks.iter().position(|t| t.id == id) {
            tasks.remove(index);
            println!("✅ Task deleted successfully.");
        } else {
            println!("❌ Task with ID {} not found.", id);
        }
    } else {
        println!("❌ Invalid ID format.");
    }
}
