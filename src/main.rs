mod commander;
mod commands;
mod ship;
mod market;


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
    let market = market::make_test_market();
    let commodity_catalog = market::make_test_commodity_catalog();
    println!("Hello, Cmdr {}.", commander.name);
    println!("You have {} credits.", commander.credits);
    commands::command_loop(&mut commander, &market, &commodity_catalog);
    println!("Goodbye, Cmdr {}.", commander.name);
}
