use std::io;

fn main() {
    println!("🧮 Simple Rust Calculator");
    println!("Available operators: (+, -, *, /)");
    println!("Enter your expression: (e.g. 5 + 3)");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let tokens: Vec<&str> = input.trim().split_whitespace().collect();

    if tokens.len() != 3 {
        println!("❌ Invalid input. Please follow the format: number operator number(e.g. 4 + 5)");
        return;
    }

    let num1: f64 = match tokens[0].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("❌ Invalid first number");
            return;
        }
    };

    let operator = tokens[1];

    let num2: f64 = match tokens[2].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("❌ Invalid second number");
            return;
        }
    };

    let result = match operator {
        "+" => add(num1, num2),
        "_" => sub(num1, num2),
        "*" => mul(num1, num2),
        "/" => {
            if num2 == 0.0 {
                println!("❌ Undefined operation. Cannot divide by zero");
                return;
            }
            div(num1, num2)
        },
        _ => {
            println!("❌ Invalid operator. Use +, -, *, or /.");
            return;
        }
    };

    println!("✅ Result: {:.2} {} {:.2} = {:.2}", num1, operator, num2, result);
}

fn add(num1: f64, num2: f64) -> f64 {
    num1 + num2
}

fn sub(num1: f64, num2: f64) -> f64 {
    num1 - num2
}

fn mul(num1: f64, num2: f64) -> f64 {
    num1 * num2
}

fn div(num1: f64, num2: f64) -> f64 {
    num1 / num2
}
