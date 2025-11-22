use std::io;
mod game;
use game::{Game, format_rating};

fn main() {
    println!("Welcome to Gamerboxd!");

    let mut games: Vec<Game> = Vec::new();
    
    loop {
        println!("**** Main Menu ****");
        println!("1. Add a new game");
        println!("2. View Your Collection");
        println!("3. View Your Favorites");
        println!("4. Add Your Favorites");
        println!("5. Exit");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        match input.trim() {
            "1" => add_game(&mut games),
            "2" => view_games(&games),
            "3" => view_favorites(&games),
            "4" => add_favorite(&mut games),
            "5" => {
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

fn add_game(games: &mut Vec<Game>) {
    println!("--- Add a New Game ---");

    let name = read_line("Enter game title:");
    let genre = read_line("Enter genre:");
    let rating_step = read_rating_step();
    let comments = read_line("Any comments?");
    let favorite = read_yes_no("Add to favorites?");

    let new_game = Game::new(name, genre, rating_step, comments, favorite);

    games.push(new_game);

    println!("Game added successfully!");
}

fn view_games(games: &[Game]) {
    println!("--- Your Collection ---");

    if games.is_empty() {
        println!("Your collection is empty!");
        return;
    }

    for game in games {
        game.display();
        println!("-----------------------");
    }
}

fn add_favorite(games: &mut Vec<Game>) {
    if games.is_empty() {
        println!("No games to update!");
        return;
    }

    println!("Enter the title of the game you want to (un)favorite:");
    let mut title = String::new();
    io::stdin().read_line(&mut title).unwrap();
    let title = title.trim();
    let mut found = false;

    for game in games.iter_mut() {
        if game.name.eq_ignore_ascii_case(title) {
            game.favorite = !game.favorite;
            println!(
                "✅ '{}' is now {}!",
                game.name,
                if game.favorite { "a favorite ❤️" } else { "no longer a favorite " }
            );
            found = true;
            break;
        }
    }

    if !found {
        println!("Game not found!");
    }

}

fn view_favorites(games : &[Game]) {
    let favorite: Vec<&Game> = games.iter().filter(|s| s.favorite).collect();
    if favorite.is_empty() {
        println!("You have no favorite games yet!");
    } else {
        for g in favorite {
            g.display();
            println!("---------------------");
        }
    }
}