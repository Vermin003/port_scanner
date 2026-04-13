# port_scanner
Port scanner written in rust

Can target a given host (default 127.0.0.1 (localhost)) and either a single or given range of ports (i.e. 1-10000)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `port_scanner` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Building
1. Install the latest stable build of [Rust](https://www.rust-lang.org/tools/install).
2. In the repository's working directory, run `cargo build` or `cargo build --release`.

## Usage
In the application directory (`./target/debug` if built from source), run `port_scanner.exe -h` to see available arguments
