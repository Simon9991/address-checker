use std::{clone, env, error::Error};

use address::Addresses;
use geocoding::MyGeocoding;

mod address;
mod geocoding;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(
            "Error: Must have one argument. Use: `cargo run path/to/file.csv`, `csv` only".into(),
        );
    }

    let mut geocoding = MyGeocoding::new().expect("API key should be an env variable");

    let old_addresses = Addresses::new(&args[1]).map_err(|e| e.to_string())?;

    // debugging print
    old_addresses.display();

    // TODO: This is for later once one call is working
    // for addr in &old_addresses.addresses {
    //     geocoding.get_address_from_google(addr.clone());
    // }

    // For now, checking only one address for testing purposes and not flood with API calls
    // TODO: remove the following once the program is finished (also maybe test with a bigger
    // sample)
    if !old_addresses.addresses.is_empty() {
        // let _ to remove warning about using Result
        let _ = geocoding
            .get_address_from_google(
                old_addresses
                    .addresses
                    .first()
                    .expect("There should be at least 1 address")
                    .clone(),
            )
            .await;
    }

    Ok(())
}
