//! Grafana CLI using the Grafana LIB crate
//! 
//! 
use grafana_lib::client::Client;

fn main() {
    // Create a client to use for cli
    let _client = Client::new(String::from("http://localhost:3000"));
}