use serde::{Serialize, Deserialize};

pub struct Game {
    pub name: String,
    pub genre: String,
    pub rating_step: u8, // 1–10, each step = 0.5★
    pub comments: String,
    pub favorite: bool,
}

pub struct PlannedGame {
    pub name: String,
    pub comments: String,
    pub favorite: bool,
}

impl Game {
    pub fn new(
        name: String,
        genre: String,
        rating_step: u8,
        comments: String,
        favorite: bool,
    ) -> Self {
        Self {
            name,
            genre,
            rating_step,
            comments,
            favorite,
        }
    }

    pub fn display(&self) {
        println!("Name: {}", self.name);
        println!("Genre: {}", self.genre);
        println!("Rating: {}", format_rating(self.rating_step));
        if self.favorite {
            println!("Favorite: ♥");
        }
        if !self.comments.is_empty() {
            println!("Comments: {}", self.comments);
        }
        println!();
    }
}

impl PlannedGame {
    pub fn new(name: String, comments: String, favorite: bool) -> Self {
        Self { name, comments, favorite }
    }

    pub fn display(&self) {
        println!("Name: {}", self.name);
        if !self.comments.is_empty() {
            println!("Comments: {}", self.comments);
        }
        println!();
    }
}

pub fn format_rating(step: u8) -> String {
    // step 1–10 → stars like "3.5/5 ★★★☆·"
    let stars_value = step as f32 / 2.0;

    let full_stars = step / 2;
    let has_half_star = step % 2 == 1;

    let mut stars = String::new();

    for _ in 0..full_stars {
        stars.push('★'); // full star
    }

    if has_half_star {
        stars.push('☆'); // half star
    }

    while stars.chars().count() < 5 {
        stars.push('·'); // empty slot
    }

    format!("{}/5 {}", stars_value, stars)
}

