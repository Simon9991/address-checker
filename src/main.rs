use std::error::Error;

use address::Addresses;
use args::Arguments;
use geocoding::MyGeocoding;

mod address;
mod args;
mod geocoding;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Arguments::new();
    let mut geocoding = MyGeocoding::new().expect("API key should be an env variable");
    let old_addresses = Addresses::new(&args.file_path_buf)?;

    for addr in &old_addresses.addresses {
        geocoding.get_address_from_google(addr).await?;
    }

    Addresses::addresses_to_csv(geocoding.address_results, &args.file_path_buf)?;

    Ok(())
}
