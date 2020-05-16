// commands.rs
use rustyline::error::ReadlineError;
use rustyline::Editor;

use super::commander::Commander;


pub fn command_loop(commander: &mut Commander) {
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
                let output = process_command(line, commander);
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


fn process_command(line: String, commander: &mut Commander) -> String {
    match line.as_ref() {
        "whoami" => commander.name.clone(),
        "rating" => commander.get_rating(),
        _ => "error".to_owned(),
    }
}
