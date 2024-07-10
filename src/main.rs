use std::{error::Error, path::PathBuf, str::FromStr};

use address::Addresses;
use args::Arguments;
use geocoding::MyGeocoding;

mod address;
mod args;
mod geocoding;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // CLI Arguments
    let args = Arguments::new();
    let file_path_buf = PathBuf::from_str(args.file_path.as_str())?;

    // Initializing the needed mod
    let mut geocoding = MyGeocoding::new().expect("API key should be an env variable");
    let old_addresses = Addresses::new(&file_path_buf)?;

    // Sending requests to the Google Maps API
    for addr in &old_addresses.addresses {
        geocoding.get_address_from_google(addr).await?;
    }

    // Serializing back into a `.csv`
    Addresses::addresses_to_csv(geocoding.address_results, &file_path_buf)?;

    Ok(())
}
