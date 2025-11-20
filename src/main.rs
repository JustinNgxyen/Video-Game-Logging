use std::io;
mod game;
use game::Game;

fn main() {
    println!("Welcome to Gamerboxd!");

    loop {
        println!("**** Main Menu ****");
        println!("1. Add a new game");
        println!("2. View Your Collection");
        println!("3. View Your Favorites");
        println!("4. Add Your Favorites");
        println!("5. Exit");

        let mut input = String::new();
        let mut games: Vec<Game> = Vec::new();
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

fn add_game(games: &mut Vec<Game>) {
    println!("coming soon!");
}

fn view_games(games: &[Game]) {
    println!("coming soon!");
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