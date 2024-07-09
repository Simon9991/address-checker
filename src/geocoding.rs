use google_maps::prelude::*;
use std::env;
use thiserror::Error;

use crate::address::Address;

#[derive(Debug)]
pub struct AddressResult {
    street_number: Option<String>,
    route: Option<String>,
    locality: Option<String>,
    administrative_area_level2: Option<String>,
    administrative_area_level1: Option<String>,
    country: Option<String>,
    postal_code: Option<String>,
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
        let address_to_search = address_obj
            .get_address_with_site_name()
            .expect("address should be found");

        dbg!(address_to_search.clone());

        let search_result = self
            .map_client
            .geocoding()
            .with_region(Region::France)
            .with_address(address_to_search)
            .execute()
            .await?;

        println!("Maps API found: {:#?}", search_result);

        let parsed_address = self.parse_geocoding_result(
            search_result
                .results
                .first()
                .expect("should get at least one result from API"),
        );
        self.address_results.push(parsed_address);

        Ok(())
    }

    fn parse_geocoding_result(&self, result: &Geocoding) -> AddressResult {
        // struct parts bc crate author committed a crime (vec as enum)
        let mut street_number = None;
        let mut route = None;
        let mut locality = None;
        let mut administrative_area_level2 = None;
        let mut administrative_area_level1 = None;
        let mut country = None;
        let mut postal_code = None;

        // parse the result into the struct

        let long_names = result
            .address_components
            .iter()
            .map(|component| component.long_name.clone());

        for (c, name) in result
            .address_components
            .iter()
            .flat_map(|component| component.types.clone())
            .zip(long_names)
        {
            match c {
                PlaceType::StreetNumber => street_number = Some(name),
                PlaceType::Route => route = Some(name),
                PlaceType::Locality => locality = Some(name),
                PlaceType::AdministrativeAreaLevel1 => administrative_area_level1 = Some(name),
                PlaceType::AdministrativeAreaLevel2 => administrative_area_level2 = Some(name),
                PlaceType::Country => country = Some(name),
                PlaceType::PostalCode => postal_code = Some(name),
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
