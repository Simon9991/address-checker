use google_maps::prelude::*;
use std::env;
use thiserror::Error;

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
    pub fn new() -> Result<Self, GeocodingError> {
        let api_key = env::var("GOOGLE_MAPS_API_KEY")?;
        let map_client = GoogleMapsClient::try_new(api_key)?;
        Ok(MyGeocoding { map_client })
    }
}
