use std::{env, error::Error};

use address::Addresses;
use geocoding::MyGeocoding;

mod address;
mod geocoding;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(
            "Error: Must have one argument. Use: `cargo run path/to/file.csv`, `csv` only".into(),
        );
    }

    let geocoding = MyGeocoding::new().expect("API key should be an env variable");

    let old_addresses = Addresses::new(&args[1]).map_err(|e| e.to_string())?;
    old_addresses.display();

    if !old_addresses.addresses.is_empty() {
        geocoding.get_address_from_google(
            old_addresses
                .addresses
                .clone()
                .first()
                .expect("There should be at least 1 address")
                .clone(),
        )
    }

    Ok(())
}
