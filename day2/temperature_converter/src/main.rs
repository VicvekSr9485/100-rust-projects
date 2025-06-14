use std::io;

fn main() {
    println!("🌡️ Temperature Converter");
    println!("1: Celcius to Fahrenheit");
    println!("2: Fahrenheit to Celcius");
    println!("3: Kelvin to Celsius");
    println!("4: Celsius to Kelvin");
    println!("5: Kelvin to Fahrenheit");
    println!("6: Fahrenheit to Kelvin");
    println!("Please select an option (1-6)");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read user input!");

    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("❌ Invalid choice. Please enter a number between 1 and 6.");
            return;
        }
    };

    match choice {
        1 => celsius_to_fahrenheit(),
        2 => fahrenheit_to_celsius(),
        3 => kelvin_to_celsius(),
        4 => celsius_to_kelvin(),
        5 => kelvin_to_fahrenheit(),
        6 => fahrenheit_to_kelvin(),
        _ => println!("❌ Invalid choice. Please enter a number between 1 and 6."),
    }
}

fn read_temperature() -> f64 {
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read input");

    match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("❌ Invalid input. Enter a valid number.");
            std::process::exit(1);
        }
    }
}

fn celsius_to_fahrenheit() {
    println!("Enter temperature in Celsius:");

    let temp = read_temperature();
    let fahrenheit = (temp * 9.0 / 5.0) + 32.0;
    println!("{:.2}ºC is {:.2}ºF", temp, fahrenheit);
}

fn fahrenheit_to_celsius() {
    println!("Enter temperature in Fahrenheit:");

    let temp = read_temperature();
    let celsius = (temp - 32.0) * 5.0 / 9.0;
    println!("{:.2}ºF is {:.2}ºC", temp, celsius);
}

fn kelvin_to_celsius() {
    println!("Enter temperature in Kelvin:");

    let temp = read_temperature();
    let celsius = temp - 273.15;
    println!("{:.2}K is {:.2}ºC", temp, celsius);
}

fn celsius_to_kelvin() {
    println!("Enter temperature in Celsius:");

    let temp = read_temperature();
    let kelvin = temp + 273.15;
    println!("{:.2}ºC is {:.2}K", temp, kelvin);
}

fn kelvin_to_fahrenheit() {
    println!("Enter temperature in Kelvin:");

    let temp = read_temperature();
    let fahrenheit = (temp - 273.15) * 9.0 / 5.0 + 32.0;
    println!("{:.2}K is {:.2}ºF", temp, fahrenheit);
}

fn fahrenheit_to_kelvin() {
    println!("Enter temperature in Fahrenheit:");

    let temp = read_temperature();
    let kelvin = (temp - 32.0) * 5.0 / 9.0 + 273.15;
    println!("{:.2}ºF is {:.2}K", temp, kelvin);
}
