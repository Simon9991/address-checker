use google_maps::prelude::*;
use std::env;
use thiserror::Error;

use crate::address::Address;

#[derive(Debug)]
pub struct MyGeocoding {
    map_client: GoogleMapsClient,
    pub address_results: Vec<Address>,
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
        let map_client = GoogleMapsClient::try_new(api_key)?.build();

        let address_results = vec![];

        Ok(MyGeocoding {
            map_client,
            address_results,
        })
    }

    /// Searches for the passed `address_obj` argument.
    /// ## Arguments
    /// `address_obj` --> an `Address` object containing the needed information
    pub async fn get_address_from_google(
        &mut self,
        address_obj: &Address,
    ) -> Result<(), GeocodingError> {
        let address_to_search = address_obj
            .get_address_with_site_name()
            .expect("address should be found");

        dbg!(&address_to_search);

        let search_result = self
            .map_client
            .geocoding()
            .with_region(Region::France)
            .with_address(address_to_search)
            .execute()
            .await?;

        println!("Maps API found: {:#?}", search_result);

        let parsed_address = Address::parse_geocoding_result(
            search_result
                .results
                .first()
                .expect("should get at least one result from API"),
            address_obj.get_site_name(),
        );
        dbg!(&parsed_address);
        self.address_results.push(parsed_address);

        Ok(())
    }
}
