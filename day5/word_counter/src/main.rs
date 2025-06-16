use std::env;
use std::fs::File;
use std::io::{self, Read};

fn main() {
    // collect command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("❌ Usage: cargo run <file_path>");
        return;
    }

    let file_path = &args[1];
    println!("📂 Reading file: {}", file_path);
    // open the file
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            println!("❌ Error opening file: {}", e);
            return;
        }
    };
    // read the file content
    let mut content = String::new();
    if let Err(e) = file.read_to_string(&mut content) {
        println!("❌ Error reading file: {}", e);
        return;
    }
    
    // count words, lines and characters
    let word_count = count_words(&content);
    let line_count = count_lines(&content);
    let char_count = count_characters(&content);
    
    println!("📊 Statistics:");
    println!("   Words: {}", word_count);
    println!("   Lines: {}", line_count);
    println!("   Characters: {}", char_count);
}

fn count_words(content: &str) -> usize {
    content.split_whitespace().count()
}

fn count_lines(content: &str) -> usize {
    // Count lines by splitting on newlines
    // Adding 1 if content doesn't end with newline and isn't empty
    if content.is_empty() {
        0
    } else {
        content.lines().count()
    }
}

fn count_characters(content: &str) -> usize {
    // Count all characters including whitespace and newlines
    content.chars().filter(|c| !c.is_whitespace()).count()
}
// This Rust program reads a file specified by the user and counts the number of words, lines, and characters in it.
// It uses the standard library to handle file I/O and string manipulation.
// The program expects a single command line argument which is the path to the file to be read.
// The program prints the total word, line, and character count of the file content to the console.
// To run the program, use the command: cargo run <file_path>
// Make sure to replace <file_path> with the actual path to the file you want to analyze.
