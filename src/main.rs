use std::{error::Error, path::PathBuf, str::FromStr, sync::Arc, time::Instant};

use address::Addresses;
use args::Arguments;
use futures::stream::{self, StreamExt};
use geocoding::MyGeocoding;
use tokio::sync::{Mutex, Semaphore};

mod address;
mod args;
mod geocoding;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Timing performance
    let start = Instant::now();

    // CLI Arguments
    let args = Arguments::new();
    let file_path_buf = PathBuf::from_str(args.file_path.as_str())?;

    // Initializing the needed mod
    let geocoding = Arc::new(Mutex::new(
        MyGeocoding::new().expect("API key should be an env variable"),
    ));
    let old_addresses = Addresses::new(&file_path_buf)?;

    // Creating a semaphore to limit concurrent requests
    let semaphore = Arc::new(Semaphore::new(10)); // Possible to adjust
                                                  // the number based on the API rate limits

    let results = stream::iter(old_addresses.addresses.iter())
        .map(|addr| {
            let gc = Arc::clone(&geocoding);
            let sp = Arc::clone(&semaphore);

            async move {
                let _permit = sp.acquire().await.unwrap();
                let mut gc = gc.lock().await;
                gc.get_address_from_google(addr).await
            }
        })
        .buffer_unordered(10)
        .collect::<Vec<_>>()
        .await;

    for res in results {
        res?;
    }

    let address_results = geocoding.lock().await.address_results.clone();
    Addresses::addresses_to_csv(address_results, &file_path_buf)?;

    // let address_results = results.into_iter().filter_map(Result::ok);

    // Sending requests to the Google Maps API
    // for addr in &old_addresses.addresses {
    //     geocoding.get_address_from_google(addr).await?;
    // }

    // Serializing back into a `.csv`
    // Addresses::addresses_to_csv(geocoding.address_results, &file_path_buf)?;

    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);

    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);

    Ok(())
}
