use std::io;

fn main() {
    println!("🗒️ Welcome to the Palindrome Checker!");
    println!("Please enter a string to check if it is a palindrome:");

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("❌ Failed to read line");

    let cleaned_input = clean_string(&input);
    if cleaned_input.is_empty() {
        println!("❌ Please enter a valid, non-empty string.");
        return;
    }
     if is_palindrome(&cleaned_input) {
        println!("✅ The string '{}' is a palindrome.", input.trim());
    } else {
        println!("❌ The string '{}' is not a palindrome.", input.trim());
    }
}

fn clean_string(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase().to_string())
        .collect::<String>()
}

fn is_palindrome(input: &str) -> bool {
    let reversed: String = input.chars().rev().collect();
    input == reversed
}
