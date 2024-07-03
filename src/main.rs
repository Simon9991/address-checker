use std::env;

use address::Addresses;

mod address;
mod geocoding;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(
            "Error: Must have one argument. Use: `cargo run path/to/file.csv`, `csv` only"
                .to_string(),
        );
    }

    let old_addresses = Addresses::new(&args[1]).map_err(|e| e.to_string())?;

    old_addresses.display();

    Ok(())
}
