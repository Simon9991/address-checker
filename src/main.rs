use std::{path::PathBuf, sync::Arc, time::Instant};

use address::{Address, Addresses};
use args::Arguments;
use futures::stream::{self, StreamExt};
use geocoding::{GeocodingError, MyGeocoding};
use tokio::sync::Semaphore;

mod address;
mod args;
mod geocoding;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Timing performance
    let start = Instant::now();

    // CLI Arguments
    let args = Arguments::new();
    let mut file_path_buf = PathBuf::new();
    file_path_buf.push(args.file_path.as_str());

    // Initializing the needed mod
    let geocoding = Arc::new(MyGeocoding::new()?);
    let old_addresses = Addresses::new(&file_path_buf)?;

    // Possible to adjust the number based on the API rate limits
    const MAX_CONCURRENT_REQUESTS: usize = 10;

    // Creating a semaphore to limit concurrent requests
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_REQUESTS));

    let results = stream::iter(old_addresses.addresses.into_iter())
        .map(|addr| {
            let gc = Arc::clone(&geocoding);
            let sp = Arc::clone(&semaphore);

            async move {
                let _permit = sp.acquire().await.unwrap();
                gc.get_address_from_google(addr).await
            }
        })
        .buffer_unordered(30)
        .collect::<Vec<Result<Address, GeocodingError>>>()
        .await;

    dbg!(&results);

    // The ones that weren't errors
    let mut found_addresses = vec![];

    if args.skip_error_check {
        found_addresses = results.into_iter().filter_map(|r| r.ok()).collect();
    } else {
        // we want to return the first error we see!
        for res in results {
            found_addresses.push(res?);
        }
    }

    Addresses::addresses_to_csv(found_addresses, &file_path_buf)?;

    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);

    Ok(())
}
