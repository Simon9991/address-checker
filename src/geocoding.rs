use google_maps::prelude::*;
use std::env;
use thiserror::Error;

use crate::address::Address;

#[derive(Debug)]
pub struct MyGeocoding {
    map_client: GoogleMapsClient,
}

#[derive(Error, Debug)]
pub enum GeocodingError {
    #[error("Environment variable error: {0}")]
    EnvVarError(#[from] std::env::VarError),

    #[error("Google Maps API error: {0}")]
    GoogleMapsError(#[from] google_maps::GoogleMapsError),
}

impl MyGeocoding {
    /// Initializes a new instance of `MyGeocoding`
    /// ## Arguments
    /// This function automatically searches for an `env` variable called `GOOGLE_MAPS_API_KEY`. It
    /// returns a custom `GeocodingError` if it is not found.
    pub fn new() -> Result<Self, GeocodingError> {
        let api_key = env::var("GOOGLE_MAPS_API_KEY")?;
        let map_client = GoogleMapsClient::try_new(api_key)?;

        Ok(MyGeocoding { map_client })
    }

    /// Searches for the passed `address_obj` argument.
    /// ## Arguments
    /// `address_obj` --> an `Address` object containing the needed information
    /// ## Returning
    /// _Not implemented yet_
    /// Returns the **non parsed** found address as a string and the lat and lng
    pub async fn get_address_from_google(&self, address_obj: Address) {
        let radius: u32 = 5000;

        // TODO: await here
        let search_result = self
            .map_client
            .text_search(address_obj.obj_to_string(), radius)
            .execute()
            .await;

        println!("{:#?}", search_result);
    }
}
