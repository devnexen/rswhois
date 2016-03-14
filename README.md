# rswhois

A very basic whois implementation in Rust.
For the moment can take as an argument a whois server or
iterate through a defined set of urls.

To build it with cargo :

- cargo build --release
- target/release/rswhois [-w (optional) whois server] hostname
