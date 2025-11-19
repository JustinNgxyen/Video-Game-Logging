use serde::{Serialize, Deserialize};

pub struct Game {
    pub name: String,
    pub genre: String,
    pub rating: f32,
    pub comments: String,
}

impl Game {
    pub fn new(name: String, genre: String, rating: f32, comments: String) -> Self {
        Self {
            name,
            genre,
            rating,
            comments,
        }
    }

    pub fn display(&self) {
        println!("{}", self.name);
        println!("Genre: {}", self.genre);
        println!("Rating: {}/5 ⭐ ", self.rating);
        println!("Comments: {}", self.comments);
    }
}