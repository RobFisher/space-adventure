// commander.rs

pub struct Commander {
    pub name: String,
    pub credits: i32,
    rating: i8,
}


impl Commander {
    pub fn new(name: String) -> Commander {
        Commander {
            name: name,
            credits: 0,
            rating: 0,
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
