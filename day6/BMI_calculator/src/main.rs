// This is a simple BMI calculator in Rust.
//BMI is calculated using the formula: weight (kg) / (height (m) * height (m))
//BMI = weight(kg) / (height(m) * height(m))
//BMI < 18.5: Underweight
//BMI 18.5 - 24.9: Normal weight
//BMI 25.0 - 29.9: Overweight
//BMI >= 30: Obesity

use std::io;

fn main() {
    println!("Welcome to the BMI Calculator!");
    println!("Please enter your weight in kilograms:");
    let mut weight_input = String::new();
    io::stdin()
        .read_line(&mut weight_input)
        .expect("Failed to read line");
    let weight: f32 = match weight_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input for weight. Please enter a valid number.");
            return;
        }
    };
    println!("Please enter your height in meters:");
    let mut height_input = String::new();
    io::stdin()
        .read_line(&mut height_input)
        .expect("Failed to read line");
    let height: f32 = match height_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input for height. Please enter a valid number.");
            return;
        }
    };
    let bmi = weight / (height * height);
    println!("Your BMI is: {:.2}", bmi);
    match bmi {
        bmi if bmi < 18.5 => println!("You are underweight."),
        bmi if bmi > 18.5 && bmi < 25.0 => println!("You have a normal weight."),
        bmi if bmi >= 25.0 && bmi < 30.0 => println!("You are overweight."),
        _ => println!("You are obese."),
    }
    println!("Thank you for using the BMI Calculator!");
}
