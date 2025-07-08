use std::io;
use rand::Rng;

fn main() {
    println!("🎮 Welcome to Rock-Paper-Scissors");
    println!("Instruction: Enter 'rock', 'paper', or 'scissors'. type 'quit' to exit");

    loop {
        println!("🪨|📃|✂️ Please enter your choice:");

        let user_choice = get_user_choice();
        if user_choice == "quit" {
            println!("Thank you for playing. Goodbye! 👋");
            break;
        }

        let computer_choice = get_computer_choice();
        println!("🤖 Computer chose: {}", computer_choice);

        match determine_winner(&user_choice, &computer_choice) {
            GameResult::UserWins => println!("🎉 You win!"),
            GameResult::ComputerWins => println!("😢 You lose!"),
            GameResult::Draw => println!("🤝 It's a draw!"),
        }
        
    }

}

fn get_user_choice() -> String {
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice = choice.trim().to_lowercase();
    match choice.as_str() {
        "rock" | "paper" | "scissors" => choice,
        "quit" => "quit".to_string(),
        _ => {
            println!("❗ Invalid choice. Please try again.");
            get_user_choice()
        }
    }
}

fn get_computer_choice() -> String {
    let choices = ["rock", "paper", "scissors"];
    let random_index = rand::thread_rng().gen_range(0..choices.len());
    choices[random_index].to_string()
}

//enum for game result
enum GameResult {
    UserWins,
    ComputerWins,
    Draw,
}
fn determine_winner(user_choice: &str, computer_choice: &str) -> GameResult {
    if user_choice == computer_choice {
        GameResult::Draw
    } else if (user_choice == "rock" && computer_choice == "scissors") ||
              (user_choice == "paper" && computer_choice == "rock") ||
              (user_choice == "scissors" && computer_choice == "paper") {
        GameResult::UserWins
    } else {
        GameResult::ComputerWins
    }
}

