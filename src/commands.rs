// commands.rs
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::str::FromStr;

use super::commander::Commander;
use super::market::Market;
use super::market::MarketAction;
use super::market::Commodity;


pub fn command_loop(commander: &mut Commander, market: &Market, commodity_catalog: &Vec<Commodity>) {
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                if line == "quit" || line == "exit" {
                    break;
                }
                let output = process_command(line, commander, market, commodity_catalog);
                println!("{}", output);
            }
            Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}


fn process_command(line: String, commander: &mut Commander, market: &Market, commodity_catalog: &Vec<Commodity>) -> String {
    let mut words = line.split_whitespace();
    let first_word = words
        .next()
        .unwrap_or("");

    match first_word {
        "whoami" => commander.name.clone(),
        "rating" => commander.get_rating(),
        "shipname" => commander.ship.name.clone(),
        "market" => market.get_price_list(commodity_catalog),
        "buy" => process_buy_command(words.collect(), commander, market, commodity_catalog),
        "sell" => process_sell_command(words.collect(), commander, market, commodity_catalog),
        "cargo" => commander.ship.get_cargo(),
        "credits" => format!("You have {} credits.", commander.credits),
        _ => "error".to_owned(),
    }
}


fn process_buy_command(words: Vec<&str>, commander: &mut Commander, market: &Market, commodity_catalog: &Vec<Commodity>) -> String {
    let error = "To buy, type: buy <quanity> <commodity name>.".to_owned();
    if words.len() == 2 {
        if let Ok(quanity) = FromStr::from_str(words[0]) {
            if let Some(commodity) = commodity_catalog.iter().find(|&x| x.name == words[1]) {
                if let Some(cost) = commander.buy(market, commodity, quanity) {
                    commander.ship.load_cargo(commodity.name.clone(), quanity);
                    format!("Bought {} {} for {}. You have {} credits remaining.",
                        quanity,
                        commodity.name,
                        cost,
                        commander.credits,
                    )
                }
                else {
                    format!("Unable to buy {} {} as price is {} per item and you have {} credits.",
                        quanity,
                        commodity.name,
                        market.get_price(commodity, MarketAction::Buy),
                        commander.credits,
                    )
                }
            } else {
                format!("Commodity {} not found in the market.", words[1])
            }
        } else {
            error
        }
    } else {
        error
    }
}


fn process_sell_command(words: Vec<&str>, commander: &mut Commander, market: &Market, commodity_catalog: &Vec<Commodity>) -> String {
    let error = "To sell, type: sell <quantity> <commodity name>.".to_owned();
    if words.len() == 2 {
        if let Ok(quantity_to_sell) = FromStr::from_str(words[0]) {
            if let Some(quantity_in_hold) = commander.ship.cargo.get(words[1]) {
                if quantity_to_sell <= *quantity_in_hold {
                    if let Some(commodity) = commodity_catalog.iter().find(|&x| x.name == words[1]) {
                        commander.ship.unload_cargo(&commodity.name, quantity_to_sell);
                        let price = commander.sell(market, commodity, quantity_to_sell);
                        format!("Sold {} {} for {} credits.", quantity_to_sell, words[1], price)
                    } else {
                        format!("{} is not sellable on this market.", words[1])
                    }
                } else {
                    format!("You only have {} of {} in the hold.", quantity_in_hold, words[1])
                }
            } else {
                format!("Commodity {} not found in the cargo hold.", words[1])
            }
        } else {
            error
        }
    } else {
        error
    }
}