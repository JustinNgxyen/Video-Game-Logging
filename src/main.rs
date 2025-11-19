use std::io;
mod game;
use game::Game;

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

fn add_game() {
    println!("coming soon!");
}

fn view_games() {
    println!("coming soon!");
}
