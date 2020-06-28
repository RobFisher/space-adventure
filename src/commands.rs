// commands.rs
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::str::FromStr;

use super::commander::Commander;
use super::market::Market;
use super::market::MarketAction;


pub fn command_loop(commander: &mut Commander, market: &Market) {
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
                let output = process_command(line, commander, market);
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


fn process_command(line: String, commander: &mut Commander, market: &Market) -> String {
    let mut words = line.split_whitespace();
    let first_word = words
        .next()
        .unwrap_or("");

    match first_word {
        "whoami" => commander.name.clone(),
        "rating" => commander.get_rating(),
        "shipname" => commander.ship.name.clone(),
        "market" => market.get_price_list(),
        "buy" => process_buy_command(words.collect(), commander, market),
        "sell" => process_sell_command(words.collect(), commander, market),
        "cargo" => commander.ship.get_cargo(),
        "credits" => format!("You have {} credits.", commander.credits),
        _ => "error".to_owned(),
    }
}


fn process_buy_command(words: Vec<&str>, commander: &mut Commander, market: &Market) -> String {
    let error = "To buy, type: buy <quanity> <commodity name>.".to_owned();
    if words.len() == 2 {
        if let Ok(quantity) = FromStr::from_str(words[0]) {
            let commodity_name = words[1].to_owned();
            if let Some(unit_price) = market.get_price(&commodity_name, MarketAction::Buy) {
                let cost = unit_price * quantity as i32;
                if let Some(credits_spent) = commander.spend(cost) {
                    commander.ship.load_cargo(&commodity_name, quantity);
                    format!("Bought {} {} for {}. You have {} credits remaining.",
                        quantity,
                        commodity_name,
                        credits_spent,
                        commander.credits,
                    )
                } else {
                    format!("Unable to buy {} {} as price is {} per item and you have {} credits.",
                        quantity,
                        commodity_name,
                        unit_price,
                        commander.credits,
                    )
                }
            } else {
                format!("Commodity {} not found in the market.", commodity_name)
            }
        }
        else {
            error
        }
    } else {
        error
    }
}


fn process_sell_command(words: Vec<&str>, commander: &mut Commander, market: &Market) -> String {
    let error = "To sell, type: sell <quantity> <commodity name>.".to_owned();
    if words.len() == 2 {
        let commodity_name = words[1].to_owned();
        if let Ok(quantity_to_sell) = FromStr::from_str(words[0]) {
            if let Some(quantity_in_hold) = commander.ship.cargo.get(&commodity_name) {
                if quantity_to_sell <= *quantity_in_hold {
                    if let Some(unit_price) = market.get_price(&commodity_name, MarketAction::Sell) {
                        commander.ship.unload_cargo(&commodity_name, quantity_to_sell);
                        let earnings = unit_price * quantity_to_sell as i32;
                        commander.earn(earnings);
                        format!("Sold {} {} for {} credits.", quantity_to_sell, commodity_name, earnings)
                    } else {
                        format!("{} is not sellable on this market.", commodity_name)
                    }
                } else {
                    format!("You only have {} of {} in the hold.", quantity_in_hold, commodity_name)
                }
            } else {
                format!("Commodity {} not found in the cargo hold.", commodity_name)
            }
        } else {
            error
        }
    } else {
        error
    }
}
