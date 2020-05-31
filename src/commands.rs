// commands.rs
use rustyline::error::ReadlineError;
use rustyline::Editor;

use super::commander::Commander;
use super::market::Market;
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
    match line.as_ref() {
        "whoami" => commander.name.clone(),
        "rating" => commander.get_rating(),
        "shipname" => commander.ship.name.clone(),
        "market" => market.get_price_list(commodity_catalog),
        _ => "error".to_owned(),
    }
}
