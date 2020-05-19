mod commander;
mod commands;
mod ship;

fn get_credits() -> i32 {
    42
}

#[test]
fn test_get_credits() {
    assert_eq!(get_credits(), 42)
}


fn buy_item(quantity: i8, price: i32) -> i32 {
    let cost = price * quantity as i32;
    println!("You bought {} for {}.", quantity, cost);
    cost
}

#[test]
fn test_buy_item() {
    assert_eq!(buy_item(3, 12), 36)
}


fn parse_args (args: impl Iterator<Item = String>) -> String
{
    let name = args.skip(1).next();
    match name {
        None => "Jameson".to_owned(),
        Some(value) => value,
    }
}

#[test]
fn test_parse_args() {
    let args = vec!["program".to_owned(), "abcde".to_owned()].into_iter();
    let name = parse_args(args);
    assert_eq!(name, "abcde");
    let args = vec![String::from("program")].into_iter();
    let name = parse_args(args);
    assert_eq!(name, "Jameson");
}


fn main() {
    let args = std::env::args();
    let name = parse_args(args);
    let mut commander = commander::Commander::new(name);
    println!("Hello, Cmdr {}.", commander.name);
    println!("Credits: {}", get_credits());
    buy_item(3, 84);
    commands::command_loop(&mut commander);
    println!("Goodbye, Cmdr {}.", commander.name);
}
