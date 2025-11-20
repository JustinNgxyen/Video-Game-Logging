use serde::{Serialize, Deserialize};

pub struct Game {
    pub name: String,
    pub genre: String,
    pub rating: f32,
    pub comments: String,
    pub favorite: bool,
}

impl Game {
    pub fn new(name: String, genre: String, rating: f32, comments: String, favorite: bool) -> Self {
        Self {
            name,
            genre,
            rating,
            comments,
            favorite,
        }
    }

    pub fn display(&self) {
        println!("{}", self.name);
        println!("Genre: {}", self.genre);
        println!("Rating: {}/5 ⭐ ", self.rating);
        println!("Comments: {}", self.comments);
        println!("Favorites: {}", if self.favorite { "Yes" } else { "No" });
    }
}