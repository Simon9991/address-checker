use std::error::Error;

use address::Addresses;

mod address;

fn main() -> Result<(), Box<dyn Error>> {
    let old_addresses = Addresses::new("old_addresses.csv")?;

    old_addresses.display();

    Ok(())
}
