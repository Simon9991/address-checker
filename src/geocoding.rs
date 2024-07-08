use google_maps::prelude::*;
use std::env;
use thiserror::Error;

use crate::address::Address;

#[derive(Debug)]
pub struct AddressResult {
    street_number: String,
    route: String,
    locality: String,
    administrative_area_level2: String,
    administrative_area_level1: String,
    country: String,
    postal_code: String,
    lat: google_maps::prelude::Decimal,
    lng: google_maps::prelude::Decimal,
}

#[derive(Debug)]
pub struct MyGeocoding {
    map_client: GoogleMapsClient,
    address_results: Vec<AddressResult>,
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
        address_obj: Address,
    ) -> Result<(), GeocodingError> {
        let address_to_search = address_obj.get_address_with_site_name();

        println!("{}", address_to_search);

        let search_result = self
            .map_client
            .geocoding()
            .with_region(Region::France)
            .with_address(address_to_search)
            .execute()
            .await?;

        println!("{:#?}", search_result);

        if let Some(result) = search_result.results.first() {
            let address_results = self.parse_geocoding_result(result);
            self.address_results.push(address_results);
        }

        Ok(())
    }

    fn parse_geocoding_result(&self, result: &Geocoding) -> AddressResult {
        let mut street_number = String::new();
        let mut route = String::new();
        let mut locality = String::new();
        let mut administrative_area_level2 = String::new();
        let mut administrative_area_level1 = String::new();
        let mut country = String::new();
        let mut postal_code = String::new();

        for component in &result.address_components {
            match component.types.first() {
                Some(PlaceType::StreetNumber) => street_number = component.long_name.clone(),
                Some(PlaceType::Route) => route = component.long_name.clone(),
                Some(PlaceType::Locality) => locality = component.long_name.clone(),
                Some(PlaceType::AdministrativeAreaLevel1) => {
                    administrative_area_level1 = component.long_name.clone()
                }
                Some(PlaceType::AdministrativeAreaLevel2) => {
                    administrative_area_level2 = component.long_name.clone()
                }
                Some(PlaceType::Country) => country = component.long_name.clone(),
                Some(PlaceType::PostalCode) => postal_code = component.long_name.clone(),
                _ => {}
            }
        }

        AddressResult {
            street_number,
            route,
            locality,
            administrative_area_level2,
            administrative_area_level1,
            country,
            postal_code,
            lat: result.geometry.location.lat,
            lng: result.geometry.location.lng,
        }
    }
}

impl AddressResult {
    pub fn parse_to_address_obj(&self) {
        todo!()
    }
}
