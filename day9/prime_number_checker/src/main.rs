use std::io;

fn main() {
    println!("Prime Number Checker");
    println!("Enter a positive integer to check if it is a prime number:");

    let number = match get_input_as_u32() {
        Some(value) => value,
        None => {
            println!("❌ Please enter a valid positive integer.");
            return;
        }
    };

    if number <= 1 {
        println!("❌ The number must be greater than 1.");
        return;
    }

    if is_prime(number) {
        println!("✅ {} is a prime number.", number);
    } else {
        println!("❌ {} is not a prime number.", number);
    }

    //Print all prime numbers up to the input number
    println!("\n🔢 Prime numbers up to {}:", number);
    let primes = primes_up_to(number);
    println!("{:?}", primes);
}

fn get_input_as_u32() -> Option<u32> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    match input.trim().parse::<u32>() {
        Ok(num) => Some(num),
        Err(_) => None,
    }
}

fn is_prime(num: u32) -> bool {
    if num <= 1 {
        return false;
    }
    if num == 2 {
        return true; // 2 is the only even prime number
    }
    if num % 2 == 0 {
        return false; // Exclude all other even numbers
    }
    let limit = (num as f64).sqrt() as u32;
    for i in 3..=limit {
        if num % i == 0 {
            return false; // Found a divisor, not prime
        }
    }
    true // No divisors found, it is prime
}

fn primes_up_to(limit: u32) -> Vec<u32> {
    let mut primes = Vec::new();
    for num in 2..=limit {
        if is_prime(num) {
            primes.push(num);
        }
    }
    primes
}
