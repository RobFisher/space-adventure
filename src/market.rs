// market.rs

// Approach:
// Maintain a database of all commodities available anywhere in the
// universe with attirbutes describing the types of economy that
// produce and them.
//
// Each market is used by different combinations of economic activity
// represeted by supply and demand for commodities produced by different
// economy tpes. To buy or sell we first ask the market about the price
// and availability of supply or demand for a commodity and this is
// calculated lazily.

use std::collections::HashMap;
use unicase::eq;


#[derive(PartialEq, Eq, Hash)]
pub enum EconomyType {
    Industrial,
    Agricultural,
    Leisure,
}


pub struct Commodity {
    pub name: String,
    pub description: String,
    pub produced_by: Vec<EconomyType>,
    pub base_price: i32,
    pub size: u32,
    pub mass: u32,
}


type CommodityCatalog = Vec<Commodity>;


pub struct Market<'a> {
    pub supply: HashMap<EconomyType, f32>,
    pub demand: HashMap<EconomyType, f32>,
    catalog: &'a CommodityCatalog,
}


pub enum MarketAction {
    Buy,
    Sell,
}


impl Market<'_> {
    pub fn get_commodity(&self, name: &String) -> Option<&Commodity> {
        self.catalog.iter().find(|&x| eq(&x.name, &name))
    }

    fn calculate_price(&self, commodity: &Commodity, action: MarketAction) -> i32 {
        let price_multiplier_by_economy_type = match action {
            MarketAction::Buy => &self.supply,
            MarketAction::Sell => &self.demand,
        };
        let result = commodity.produced_by
            .iter()
            .map(
                |s|
                    price_multiplier_by_economy_type
                        .get(s)
                        .unwrap_or(&1.0)
                        * commodity.base_price as f32
                        * match action {
                            MarketAction::Buy => 2.0, // TODO: come up with a better market model than this
                            MarketAction::Sell => 1.0,
                        }
                )
                .max_by(|a, b| a.partial_cmp(b).expect("Tried to compare a NaN"));
        result.unwrap_or(1.0) as i32 // TODO: figure out why result is an Option
    }

    pub fn get_price(&self, commodity_name: &String, action: MarketAction) -> Option<i32> {
        if let Some(commodity) = self.get_commodity(commodity_name) {
            Some(self.calculate_price(commodity, action))
        }
        else {
            None
        }
    }

    pub fn get_price_list(&self) -> String {
        self.catalog
            .iter()
            .map(
                |s| format!("{}: Buy Price: {} Sell Price: {}",
                s.name,
                self.calculate_price(s, MarketAction::Buy),
                self.calculate_price(s, MarketAction::Sell),
            ))
            .collect::<Vec<String>>()
            .join("\n")
    }
}


pub fn make_test_commodity_catalog() -> CommodityCatalog {
    // make some test commodities.
    // TODO: make these from a configuration file
    let food = Commodity {
        name: "Food".to_owned(),
        description: "Fresh, tasty ingredients.".to_owned(),
        produced_by: vec![EconomyType::Agricultural],
        base_price: 100,
        size: 1,
        mass: 1,
    };
    let computers = Commodity {
        name: "Computers".to_owned(),
        description: "The latest off-the-shelf computing hardware.".to_owned(),
        produced_by: vec![EconomyType::Industrial],
        base_price: 1000,
        size: 1,
        mass: 1,
    };
    let games = Commodity {
        name: "Games".to_owned(),
        description: "Digital licenses for the latest AAA game titles.".to_owned(),
        produced_by: vec![EconomyType::Leisure],
        base_price: 800,
        size: 0,
        mass: 0,
    };
    vec![food, computers, games]
}


pub fn make_test_market(catalog: &CommodityCatalog) -> Market {
    // make a test market.
    // TODO: make a function that generate this procedurally
    // based on parameters about the economy in a location.
    let mut supply_map = HashMap::new();
    supply_map.insert(EconomyType::Industrial, 0.9);
    supply_map.insert(EconomyType::Leisure, 0.7);
    let mut demand_map = HashMap::new();
    demand_map.insert(EconomyType::Agricultural, 1.3);
    demand_map.insert(EconomyType::Industrial, 1.1);
    Market {
        supply: supply_map,
        demand: demand_map,
        catalog: catalog,
    }
}
