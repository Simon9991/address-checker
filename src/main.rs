use std::{env, error::Error};

use address::Addresses;
use geocoding::{GeocodingError, MyGeocoding};

mod address;
mod geocoding;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(
            "Error: Must have one argument. Use: `cargo run path/to/file.csv`, `csv` only".into(),
        );
    }

    let geocoding = MyGeocoding::new()?;

    let old_addresses = Addresses::new(&args[1]).map_err(|e| e.to_string())?;
    old_addresses.display();
    Ok(())

    // match MyGeocoding::new() {
    //     Ok(geocoding) => {
    //         // Use the geocoding instance
    //     }
    //     Err(e) => match e {
    //         GeocodingError::EnvVarError(env_err) => {
    //             return Err("Environment variable error".into());
    //         }
    //         GeocodingError::GoogleMapsError(maps_err) => {
    //             return Err("Google Maps API error".into());
    //         }
    //     },
    // }
}
