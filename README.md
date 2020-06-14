# Space Adventure
Space Adventure is the start of a text space adventure game written in Rust. It is heavily influenced by
Bell & Braben's Elite.

# Status
I am learning Rust and figuring out the game design. There is no game loop yet.

# Usage
## Build
`cargo build --release`

## Run
`cargo run <commander name>`
or
```
cd target/release
./space-adventure <commander-name>
```

## Commands
- market: display market price list
- rating: display commander rating
- shipname: display ship name
- whoami: display commander name
- buy quanity commodity-name: buy a commodity from the market
- cargo: list the contents of the ship's cargo hold

Exit with either quit or exit.
