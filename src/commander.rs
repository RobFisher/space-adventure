// commander.rs

use super::ship::Ship;

pub struct Commander {
    pub name: String,
    pub credits: i32,
    rating: i8,
    pub ship: Ship,
}


impl Commander {
    pub fn new(name: String) -> Commander {
        Commander {
            name: name,
            credits: 0,
            rating: 0,
            ship: Ship::new("My first ship".to_owned()),
        }
    }

    pub fn get_rating(&mut self) -> String {
        match self.rating {
            0 => "rookie".to_owned(),
            1 => "novice".to_owned(),
            _ => "*classified*".to_owned(),
        }
    }
}
