use std::io;
mod game;
use game::{Game, format_rating};

fn main() {
    println!("Welcome to Gamerboxd!");

    loop {
        println!("**** Main Menu ****");
        println!("1. Add a new game");
        println!("2. View Your Collection");
        println!("3. Exit");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        match input.trim() {
            "1" => add_game(),
            "2" => view_games(),
            "3" => {
                println!("Bye!");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn read_line(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn read_yes_no(prompt: &str) -> bool {
    loop {
        let answer = read_line(&format!("{} (y/n):", prompt));
        let lower = answer.to_lowercase();
        if lower == "y" || lower == "yes" {
            return true;
        } else if lower == "n" || lower == "no" {
            return false;
        } else {
            println!("Please enter y or n.");
        }
    }
}

fn read_rating_step() -> u8 {
    println!("Rating is out of 5 stars.");
    println!("You will enter a number from 1 to 10.");
    println!("1 = 0.5★, 2 = 1.0★, 3 = 1.5★, ... 10 = 5.0★");

    loop {
        let text = read_line("Enter rating step (1-10):");
        match text.parse::<u8>() {
            Ok(n) if n >= 1 && n <= 10 => return n,
            _ => println!("Please type a number between 1 and 10."),
        }
    }
}

fn add_game() {
    println!("coming soon!");
}

fn view_games() {
    // demo game to show the rating system
    let demo_game = Game::new(
        "The Legend of Zelda".to_string(), // name
        "Adventure".to_string(),           // genre
        7,                                 // rating_step (7 -> 3.5 stars)
        "Great game with lots of exploration.".to_string(), // comments
        true,                              // favorite
    );

    println!("--- Your Collection (demo) ---");
    demo_game.display();
}
