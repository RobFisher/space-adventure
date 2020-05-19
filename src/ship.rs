// ship.rs


pub struct Ship {
    pub name: String,
    pub cargo_space: i32,
    pub hull_damage: i32,
}


impl Ship {
    pub fn new(name: String) -> Ship {
        Ship {
            name: name,
            cargo_space: 30,
            hull_damage: 0,
        }
    }
}
