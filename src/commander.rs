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
            credits: 1000,
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

    pub fn spend(&mut self, amount: i32) -> Option<i32> {
        // TODO: decide what to do if passed a negative number
        if amount <= self.credits {
            self.credits -= amount;
            Some(amount)
        } else {
            None
        }
    }

    pub fn earn(&mut self, amount: i32) {
        // TODO: think about overflow handling (only real reason for this method)
        self.credits += amount;
    }
}
