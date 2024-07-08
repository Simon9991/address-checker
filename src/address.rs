use csv::ReaderBuilder;
use serde::Deserialize;
use std::{error::Error, f64, fmt, fs::File, io::BufReader, path::Path};

#[derive(Clone, Debug, PartialEq, PartialOrd, Deserialize)]
pub struct Address {
    name: String,
    address: String,
    city: String,
    zip: String,
    administrative_area_level1: String,
    administrative_area_level2: String,
    lat: f64,
    lng: f64,
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

    pub fn generate_diff_csv(
        &self,
        differences: &[(Address, Address)],
        output_path: &str,
    ) -> Result<(), Box<dyn Error>> {
        let mut wtr = csv::Writer::from_path(output_path)?;

        wtr.write_record([
            "name",
            "old_address",
            "old_city",
            "old_zip",
            "old_lat",
            "old_lng",
            "new_address",
            "new_city",
            "new_zip",
            "new_lat",
            "new_lng",
        ])?;

        for (old, new) in differences {
            wtr.write_record([
                &old.name,
                &old.address,
                &old.city,
                &old.zip,
                &old.lat.to_string(),
                &old.lng.to_string(),
                &new.address,
                &new.city,
                &new.zip,
                &new.lat.to_string(),
                &new.lng.to_string(),
            ])?;
        }

        wtr.flush()?;
        Ok(())
    }

    pub fn display(&self) {
        for (index, address) in self.addresses.iter().enumerate() {
            println!("Address {}: {}", index + 1, address);
        }
    }
}

impl Address {
    /// This function is not complete yet, probably missing more details
    pub fn obj_to_string(&self) -> String {
        let str = self.address.clone()
            + ", "
            + self.city.clone().as_str()
            + ", "
            + self.zip.clone().as_str();

        str
    }

    pub fn get_address_with_site_name(&self) -> String {
        let str = self.name.clone()
            + ", "
            + self.address.clone().as_str()
            + ", "
            + self.city.clone().as_str()
            + ", "
            + self.zip.clone().as_str();

        str
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Site: {}, Street: {}, City: {}, ZIP: {}, Area 1: {}, Area 2: {}, Lat: {}, Lng: {}",
            self.name,
            self.address,
            self.city,
            self.zip,
            self.administrative_area_level1,
            self.administrative_area_level2,
            self.lat,
            self.lng
        )
    }
}
