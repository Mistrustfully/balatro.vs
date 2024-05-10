# balatro.vs

An open source multiplayer mod for the card game Balatro!
Patches the game using [Lovely](https://github.com/ethangreen-dev/lovely-injector/), and uses [Rust](https://www.rust-lang.org/) for the server.

## Installation

1. Setup [Lovely](https://github.com/ethangreen-dev/lovely-injector/)
2. Go to the [releases](https://github.com/Mistrustfully/balatro.vs/releases) and unzip the newest release into your mods folder.

Currently there are no offical servers, so you'll also have to self host the server yourself! See [Running the web server](#running-the-web-server).

## Running the web server

Requires Rust and Cargo to be installed.

1. Run `cargo run -- server`
2. Port forward port 8080

## Building

1. Run `cargo run -- build`
2. Optionally, run `cargo run -- run` to have the game start up after your build is finished.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
