// ship.rs

use std::collections::HashMap;


pub struct Ship {
    pub name: String,
    pub cargo_space: i32,
    pub hull_damage: i32,
    pub cargo: HashMap<String, u32>,
}


impl Ship {
    pub fn new(name: String) -> Ship {
        Ship {
            name: name,
            cargo_space: 30,
            hull_damage: 0,
            cargo: HashMap::new(),
        }
    }

    pub fn load_cargo(&mut self, name: &String, quantity: u32) {
        let existing_quantity = self.cargo.entry(name.clone()).or_insert(0);
        *existing_quantity += quantity;
    }

    pub fn unload_cargo(&mut self, name: &String, quantity: u32) {
        let cargo_entry = self.cargo.get_mut(name);
        match cargo_entry {
            Some(c) => {
                if *c >= quantity {
                    *c -= quantity;
                }
                if *c == 0 {
                    self.cargo.remove(name);
                }
            }
            None => {}
        }
    }

    pub fn get_cargo(&self) -> String {
        if self.cargo.len() == 0 {
            "Cargo hold is empty.".to_owned()
        } else {
            self.cargo
                .iter()
                .map(|(name,quantity)| format!("{}: {}", name, quantity))
                .collect::<Vec<String>>()
                .join("\n")
        }
    }
}
