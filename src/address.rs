use csv::ReaderBuilder;
use serde::Deserialize;
use std::{error::Error, fs::File, io::BufReader, path::Path};

#[derive(Clone, Debug, PartialEq, PartialOrd, Deserialize)]
pub struct Address {
    name: Option<String>,
    address: Option<String>,
    city: Option<String>,
    zip: Option<String>,
    administrative_area_level1: Option<String>,
    administrative_area_level2: Option<String>,
    lat: Option<google_maps::prelude::Decimal>,
    lng: Option<google_maps::prelude::Decimal>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Addresses {
    pub addresses: Vec<Address>,
}

impl Addresses {
    pub fn new(path_to_file: &str) -> Result<Self, Box<dyn Error>> {
        let path = Path::new(path_to_file);
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut csv_reader = ReaderBuilder::new().has_headers(true).from_reader(reader);

        let mut addresses = Vec::new();

        for result in csv_reader.deserialize() {
            let address: Address = result?;
            addresses.push(address);
        }

        Ok(Addresses { addresses })
    }

    pub fn generate_diff_csv(&self) -> Result<(), Box<dyn Error>> {
        todo!()
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
            self.name.clone()?,
            self.address.clone()?,
            self.city.clone()?,
            self.zip.clone()?,
            self.administrative_area_level1.clone()?,
            self.administrative_area_level2.clone()?,
            self.lat?,
            self.lng?,
        ))
    }

    pub fn get_address_with_site_name(&self) -> Option<String> {
        Some(format!(
            "{}, {}, {}, {}",
            self.name.clone()?,
            self.address.clone()?,
            self.city.clone()?,
            self.zip.clone()?
        ))
    }
}
