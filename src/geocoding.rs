use google_maps::prelude::*;
use std::env;
use thiserror::Error;

use crate::address::Address;

#[derive(Clone, Debug)]
pub struct MyGeocoding {
    map_client: GoogleMapsClient,
}

#[derive(Error, Debug)]
pub enum GeocodingError {
    #[error("Environment variable error: {0}")]
    EnvVarError(#[from] std::env::VarError),

    #[error("Google Maps API error: {0}")]
    GoogleMapsError(#[from] google_maps::GoogleMapsError),

    #[error("Address not found in the original file")]
    FileAddressNotFound,

    #[error("No result from Google Maps API")]
    NoApiResult,
}

impl MyGeocoding {
    /// Initializes a new instance of `MyGeocoding`
    /// ## Arguments
    /// This function automatically searches for an `env` variable called `GOOGLE_MAPS_API_KEY`. It
    /// returns a custom `GeocodingError` if it is not found.
    pub fn new() -> Result<Self, GeocodingError> {
        let api_key = env::var("GOOGLE_MAPS_API_KEY")?;
        let map_client = GoogleMapsClient::try_new(api_key)?.build();

        Ok(MyGeocoding { map_client })
    }

    /// Searches for the passed `address_obj` argument.
    /// ## Arguments
    /// `address_obj` --> an `Address` object containing the needed information
    pub async fn get_address_from_google(
        &self,
        address_obj: Address,
    ) -> Result<Address, GeocodingError> {
        let address_to_search = address_obj
            .get_address_with_group_name()
            .ok_or(GeocodingError::FileAddressNotFound)?;

        let search_result = self
            .map_client
            .geocoding()
            .with_region(Region::France)
            .with_address(address_to_search)
            .execute()
            .await?;

        let parsed_address = Address::parse_geocoding_result(
            search_result
                .results
                .first()
                .ok_or(GeocodingError::NoApiResult)?,
            address_obj,
        );

        Ok(parsed_address)
    }
}
