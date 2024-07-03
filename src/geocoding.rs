use google_maps::prelude::*;
use google_maps::{geocoding::Geocoding, ClientSettings};
use std::env;

#[derive(Debug)]
pub struct MyGeocoding {
    map_client: GoogleMapsClient,
}

impl MyGeocoding {
    pub fn new() -> Result<Self, Erra> {
        let google_api_key = env::var("GOOGLE_MAPS_API_KEY");

        match env::var("GOOGLE_MAPS_API_KEY") {
            Ok(api_key) => {
                let map_client = GoogleMapsClient::new(api_key);
                return Ok(MyGeocoding { map_client });
            }
            Err(err) => return Err(err.to_string()),
        }
    }
}
