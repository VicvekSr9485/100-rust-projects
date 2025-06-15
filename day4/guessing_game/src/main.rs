use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    loop {
        println!("🎯 Welcome to the Guessing Game!");
        println!("I have selected a number between 1 and 100. Can you guess it?");
        println!("You have 5 tries to guess the number!");
        
        let secret_number = rand::thread_rng().gen_range(1..=100);
        let mut trials_left = 5;

        while trials_left > 0 {
            println!("\nTries remaining: {}", trials_left);
            println!("Please input your guess:");

            let mut guess = String::new();
            io::stdin().read_line(&mut guess).expect("Failed to read line");
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("That's not a valid number! Please try again.");
                    continue;
                }
            };

            println!("You guessed: {}", guess);

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small! Try again."),
                Ordering::Greater => println!("Too big! Try again."),
                Ordering::Equal => {
                    println!("🎉 Congratulations! You guessed the number!");
                    break;
                }
            }

            trials_left -= 1;
            if trials_left == 0 {
                println!("💔 Game Over! The number was {}.", secret_number);
            }
        }

        println!("\nWould you like to play again? (y/n)");
        let mut play_again = String::new();
        io::stdin().read_line(&mut play_again).expect("Failed to read line");
        
        if play_again.trim().to_lowercase() != "y" {
            println!("Thanks for playing the Guessing Game! 🎮");
            break;
        }
    }
}
