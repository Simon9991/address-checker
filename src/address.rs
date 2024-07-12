use csv::ReaderBuilder;
use google_maps::{geocoding::Geocoding, prelude::Decimal, PlaceType};
use num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    fs::{self, File},
    io::BufReader,
    path::Path,
};
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Address {
    #[serde(rename = "site")]
    site_name: Option<String>,
    #[serde(rename = "group")]
    group_name: Option<String>,

    #[serde(rename(deserialize = "address", serialize = "old_address"))]
    old_full_address: Option<String>,
    #[serde(rename(serialize = "new_address"), skip_deserializing)]
    new_full_address: Option<String>,

    #[serde(rename(deserialize = "city", serialize = "old_city"))]
    old_locality: Option<String>,
    #[serde(rename(serialize = "new_city"), skip_deserializing)]
    new_locality: Option<String>,

    #[serde(rename(deserialize = "zip", serialize = "old_zip"))]
    old_postal_code: Option<String>,
    #[serde(rename(serialize = "new_zip"), skip_deserializing)]
    new_postal_code: Option<String>,

    #[serde(rename(
        deserialize = "administrative_area_level1",
        serialize = "old_administrative_area_level1"
    ))]
    old_administrative_area_level1: Option<String>,
    #[serde(
        rename(serialize = "new_administrative_area_level1"),
        skip_deserializing
    )]
    new_administrative_area_level1: Option<String>,

    #[serde(rename(
        deserialize = "administrative_area_level2",
        serialize = "old_administrative_area_level2"
    ))]
    old_administrative_area_level2: Option<String>,
    #[serde(
        rename(serialize = "new_administrative_area_level2"),
        skip_deserializing
    )]
    new_administrative_area_level2: Option<String>,

    #[serde(rename(deserialize = "lat", serialize = "old_lat"), skip_serializing)]
    old_lat: google_maps::prelude::Decimal,
    #[serde(rename(deserialize = "lng", serialize = "old_lng"), skip_serializing)]
    old_lng: google_maps::prelude::Decimal,

    #[serde(rename(serialize = "new_lat"), skip_deserializing)]
    new_lat: google_maps::prelude::Decimal,
    #[serde(rename(serialize = "new_lng"), skip_deserializing)]
    new_lng: google_maps::prelude::Decimal,

    // Fields for geocoding results
    #[serde(skip_deserializing, skip_serializing)]
    street_number: Option<String>,
    #[serde(skip_deserializing, skip_serializing)]
    route: Option<String>,
    #[serde(skip_deserializing, skip_serializing)]
    country: Option<String>,
    #[serde(skip_deserializing)]
    meter_distance: f64,
}

#[derive(Error, Debug)]
pub enum AddressError {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("File not found: {0}")]
    FileNotFound(std::path::PathBuf),

    #[error("CSV error: {0}")]
    CsvError(#[from] csv::Error),

    #[error("Invalid CSV format")]
    InvalidCsvFormat,

    #[error("Failed to deserialize address")]
    DeserializationError,

    #[error("Invalid original file name")]
    InvalidOriginalFileName,

    #[error("Failed to convert the original file name into a valid string")]
    FileNameConversionFailed,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Addresses {
    pub addresses: Vec<Address>,
}

impl Addresses {
    pub fn new(path_to_file: &Path) -> Result<Self, AddressError> {
        let file = File::open(path_to_file).map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                AddressError::FileNotFound(path_to_file.to_path_buf())
            } else {
                AddressError::IoError(e)
            }
        })?;
        let reader = BufReader::new(file);

        let mut csv_reader = ReaderBuilder::new().has_headers(true).from_reader(reader);

        let mut addresses = vec![];

        for result in csv_reader.deserialize() {
            let address: Address = result.map_err(|_| AddressError::DeserializationError)?;
            addresses.push(address);
        }

        if addresses.is_empty() {
            return Err(AddressError::InvalidCsvFormat);
        }

        Ok(Addresses { addresses })
    }

    pub fn addresses_to_csv(
        adresses: Vec<Address>,
        original_file_path: &Path,
    ) -> Result<(), AddressError> {
        // 1. Checking if the the directory `./results/` exist, if not creates it
        // 1. Creating the output file path.
        fs::create_dir_all("./results")?;
        let new_file_path = format!(
            "./results/{}_gmaps_version.csv",
            original_file_path
                .file_stem()
                .ok_or(AddressError::InvalidOriginalFileName)?
                .to_str()
                .ok_or(AddressError::FileNameConversionFailed)?
        );

        // 2. Create a `csv::Writer::from_path` with the `./results/` + `new_file_name`
        let mut writer = csv::Writer::from_path(new_file_path)?;

        // 3. Write the headers
        // 3. Looping through the `self.addresses` vector and write with the csv writer
        for addr in adresses.iter() {
            writer.serialize(addr)?;
        }

        // 4. Saving the changes to the file
        writer.flush()?;

        Ok(())
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fields = [
            &self.site_name,
            &self.old_full_address,
            &self.old_locality,
            &self.old_postal_code,
            &self.old_administrative_area_level1,
            &self.old_administrative_area_level2,
        ];

        for field in fields {
            if let Some(t) = field {
                f.write_str(format!("{} ", t.to_string().as_str()).as_str())?;
            }
        }

        f.write_str(format!("{} {}", self.old_lat, self.old_lng).as_str())?;

        Ok(())
    }
}

impl Address {
    pub fn get_address_with_group_name(&self) -> Option<String> {
        Some(format!(
            "{}, {}, {}, {}",
            self.group_name.as_ref()?,
            self.old_full_address.as_ref()?,
            self.old_locality.as_ref()?,
            self.old_postal_code.as_ref()?
        ))
    }

    pub fn parse_geocoding_result(result: &Geocoding, address_obj: Address) -> Address {
        // struct parts bc crate author committed a crime (vec as enum)
        let mut street_number = None;
        let mut route = None;
        let mut locality = None;
        let mut administrative_area_level2 = None;
        let mut administrative_area_level1 = None;
        let mut country = None;
        let mut postal_code = None;
        let new_lat = result.geometry.location.lat;
        let new_lng = result.geometry.location.lng;

        for component in &result.address_components {
            for type_ in &component.types {
                match type_ {
                    PlaceType::StreetNumber => street_number = Some(component.long_name.clone()),
                    PlaceType::Route => route = Some(component.long_name.clone()),
                    PlaceType::Locality => locality = Some(component.long_name.clone()),
                    PlaceType::AdministrativeAreaLevel1 => {
                        administrative_area_level1 = Some(component.long_name.clone())
                    }
                    PlaceType::AdministrativeAreaLevel2 => {
                        administrative_area_level2 = Some(component.long_name.clone())
                    }
                    PlaceType::Country => country = Some(component.long_name.clone()),
                    PlaceType::PostalCode => postal_code = Some(component.long_name.clone()),
                    _ => {}
                }
            }
        }

        let full_address = match (street_number.as_ref(), route.as_ref()) {
            (Some(num), Some(street)) => Some(format!("{} {}", num, street)),
            (None, Some(street)) => Some(street.clone()),
            _ => None,
        };

        // Encapsulates the `unwraps` due to required Decimal trait conversion :D
        let make_geoutils_location = |lat: Decimal, lon: Decimal| {
            geoutils::Location::new(lat.to_f64().unwrap(), lon.to_f64().unwrap())
        };

        let original_location = make_geoutils_location(address_obj.old_lat, address_obj.old_lng);
        let new_location = make_geoutils_location(new_lat, new_lng);
        let distance = original_location.distance_to(&new_location).unwrap();

        Address {
            street_number,
            new_full_address: full_address,
            route,
            new_locality: locality,
            new_administrative_area_level2: administrative_area_level2,
            new_administrative_area_level1: administrative_area_level1,
            country,
            new_postal_code: postal_code,
            new_lat,
            new_lng,
            meter_distance: distance.meters(),
            ..address_obj
        }
    }
}
