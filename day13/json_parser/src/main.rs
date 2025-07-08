use std::env;
use std::fs;
use serde_json::Value;

fn main() {
    // Get the file path from command line argumemnts
    let args: Vec<String> = env::args().collect();

    if args.len() != 2{
        eprintln!("Usage: cargo run <path_to_json_file>");
        return;
    }

    let file_path = &args[1];

    match fs::read_to_string(file_path) {
        Ok(data) => {
            // Parse the JSON data
            match serde_json::from_str::<Value>(&data) {
                Ok(json_value) => {
                    // Print the parsed JSON value
                    println!("✅ JSON Passed: {}", serde_json::to_string_pretty(&json_value).unwrap());
                }
                Err(e) => {
                    eprintln!("Error parsing JSON: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
}
