// commander.rs

use super::ship::Ship;
use super::market::Market;
use super::market::MarketAction;
use super::market::Commodity;


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

    pub fn buy(&mut self, market: &Market, commodity: &Commodity, qty: u32) -> Option<i32> {
        let price = market.get_price(commodity, MarketAction::Buy) * qty as i32;
        let mut spent: Option<i32> = None;
        if price <= self.credits {
            self.credits -= price;
            spent = Some(price);
        }
        spent
    }

    pub fn sell(&mut self, market: &Market, commodity: &Commodity, qty: u32) -> i32 {
        let price = market.get_price(commodity, MarketAction::Sell) * qty as i32;
        self.credits += price;
        price
    }
}
