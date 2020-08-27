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
- credits: display credits balance
- rating: display commander rating
- shipname: display ship name
- whoami: display commander name
- buy quanity commodity-name: buy a commodity from the market
- sell quantity commodity-name: sell a commodity to the market
- cargo: list the contents of the ship's cargo hold
- undock: if docked, start undocking procedure
- dock: if undocked, start docking procedure
- status: display ship status

Exit with either quit or exit.
