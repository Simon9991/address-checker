use std::{env, error::Error, path::PathBuf, str::FromStr};

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

    let file_path = PathBuf::from_str(&args[1])?;

    let mut geocoding = MyGeocoding::new().expect("API key should be an env variable");

    // TODO: Change to PathBuf instead of String for new
    let old_addresses = Addresses::new(&args[1]).map_err(|e| e.to_string())?;

    for addr in &old_addresses.addresses {
        geocoding.get_address_from_google(addr).await?;
    }

    Addresses::addresses_to_csv(geocoding.address_results, &file_path)?;

    Ok(())
}
