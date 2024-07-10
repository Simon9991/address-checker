use csv::ReaderBuilder;
use google_maps::{geocoding::Geocoding, PlaceType};
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs::{self, File},
    io::BufReader,
    path::Path,
};

#[derive(Clone, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Address {
    name: Option<String>,

    #[serde(rename = "address")]
    full_address: Option<String>,

    #[serde(rename = "city")]
    locality: Option<String>,

    #[serde(rename = "zip")]
    postal_code: Option<String>,

    administrative_area_level1: Option<String>,
    administrative_area_level2: Option<String>,

    lat: google_maps::prelude::Decimal,
    lng: google_maps::prelude::Decimal,

    // Fields for geocoding results
    #[serde(skip_deserializing, skip_serializing)]
    street_number: Option<String>,
    #[serde(skip_deserializing, skip_serializing)]
    route: Option<String>,
    #[serde(skip_deserializing, skip_serializing)]
    country: Option<String>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Addresses {
    pub addresses: Vec<Address>,
}

impl Addresses {
    pub fn new(path_to_file: &str) -> Result<Self, Box<dyn Error>> {
        let path = Path::new(path_to_file);
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut csv_reader = ReaderBuilder::new().has_headers(true).from_reader(reader);

        let mut addresses = vec![];

        for result in csv_reader.deserialize() {
            let address: Address = result?;
            addresses.push(address);
        }

        Ok(Addresses { addresses })
    }

    pub fn addresses_to_csv(
        adresses: Vec<Address>,
        original_file_path: &Path,
    ) -> Result<(), Box<dyn Error>> {
        // 1. Checking if the the directory `./results/` exist, if not creates it
        // 1. Creating the output file path.
        fs::create_dir_all("./results")?;
        // Note: I should maybe stop to use `.expect()` everywhere but...
        let new_file_path = format!(
            "./results/{}_gmaps_version.csv",
            original_file_path
                .file_stem()
                .expect("original file name should be valid")
                .to_str()
                .expect("str conversion from PathBuf should work")
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

    pub fn display(&self) {
        for (index, address) in self.addresses.iter().enumerate() {
            println!(
                "Address {}: {}",
                index + 1,
                address
                    .get_formatted_address()
                    .expect("original file address should be correct")
            );
        }
    }
}

impl Address {
    pub fn get_formatted_address(&self) -> Option<String> {
        Some(format!(
            "{}, {}, {}, {}, {}, {}, {}, {}",
            self.name.as_ref()?,
            self.full_address.as_ref()?,
            self.locality.as_ref()?,
            self.postal_code.as_ref()?,
            self.administrative_area_level1.as_ref()?,
            self.administrative_area_level2.as_ref()?,
            self.lat,
            self.lng,
        ))
    }

    pub fn get_address_with_site_name(&self) -> Option<String> {
        Some(format!(
            "{}, {}, {}, {}",
            self.name.as_ref()?,
            self.full_address.as_ref()?,
            self.locality.as_ref()?,
            self.postal_code.as_ref()?
        ))
    }

    pub fn get_site_name(&self) -> Option<String> {
        self.name.clone()
    }

    pub fn parse_geocoding_result(result: &Geocoding, site_name: Option<String>) -> Address {
        // struct parts bc crate author committed a crime (vec as enum)
        let mut street_number = None;
        let mut route = None;
        let mut locality = None;
        let mut administrative_area_level2 = None;
        let mut administrative_area_level1 = None;
        let mut country = None;
        let mut postal_code = None;

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

        Address {
            name: site_name,
            street_number,
            full_address,
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
