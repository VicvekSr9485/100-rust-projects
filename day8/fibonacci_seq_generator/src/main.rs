use std::io;

fn main() {
    println!("Fibonacci Sequence Generator");
    println!("Enter the number of terms in the Fibonacci sequence you want to generate:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("❌ Failed to input");

    let num_terms = match get_input_as_u32(&input) {
        Some(value) => value,
        None => {
            println!("❌ Please enter a valid positive integer.");
            return;
        }
    };
     if num_terms == 0 {
        println!("❌ The number of terms must be greater than zero.");
        return;
     };

    let fibonacci_sequence = generate_fibonacci(num_terms);
    println!("✅ Fibonacci sequence with ({} terms): {:?}", num_terms, fibonacci_sequence);
}

fn get_input_as_u32(input: &str) -> Option<u32> {
    match input.trim().parse::<u32>() {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}

fn generate_fibonacci(n: u32) -> Vec<u64> {
    let mut sequence = Vec::new();

    if n >= 1 {
        sequence.push(0);
    }
    if n >= 2 {
        sequence.push(1);
    }
    for i in 2..n {
        let next_value = sequence[i as usize - 1] + sequence[i as usize - 2];
        sequence.push(next_value);
    }
    sequence
}
