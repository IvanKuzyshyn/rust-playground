pub mod countries_serializer {
    use clap::error::ErrorKind;
    use serde::Deserialize;
    use serde_json;
    use std::collections::HashMap;
    use std::error::Error as StandardError;
    use std::fmt::{Display, Formatter, Result};
    use std::fs::File;
    use std::io;
    use std::io::{Error, Read};

    const ASSETS_DIR: &str = "./assets/";
    const COUNTRIES_JSON: &str = "countries.json";

    #[derive(Deserialize, Debug, Clone)]
    pub struct CountryNativeName {
        common: String,
        official: String,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct CountryName {
        common: String,
        official: String,
        native: HashMap<String, CountryNativeName>,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct Country {
        name: CountryName,
        cca2: String,
        ccn3: String,
        cca3: String,
        region: String,
        subregion: String,
    }

    struct CountryList {
        list: Vec<Country>,
    }

    #[derive(Debug)]
    enum CustomError {
        NotFound(String),
    }

    impl StandardError for CustomError {}

    impl Display for CustomError {
        fn fmt(&self, f: &mut Formatter) -> Result {
            match *self {
                CustomError::NotFound(ref name) => {
                    write!(f, "Country with name {} not found!", name)
                }
            }
        }
    }

    impl CountryList {
        pub fn new() -> Self {
            CountryList { list: Vec::new() }
        }
        pub fn read_from_file(&mut self, file_path: &str) -> Result<(), Error> {
            let list_as_string_result = read_file_as_string(file_path);

            if list_as_string_result.is_err() {
                return Err(list_as_string_result.unwrap_err());
            }

            let list_as_string = list_as_string_result.unwrap();
            self.list = serialize_json(&list_as_string);

            Ok(())
        }

        pub fn get_list(self) -> Vec<Country> {
            self.list
        }

        pub fn is_empty_list(self) -> bool {
            self.list.is_empty()
        }

        pub fn find_country_by_name(self, name: &str) -> Result<Country, CustomError> {
            let found_country = self
                .list
                .iter()
                .find(|country| country.name.common.eq(name));

            if found_country.is_none() {
                return Err(ErrorKind::as_str(CustomError::NotFound(name.to_string()));
            }

            Ok(())
        }

        fn serialize_json(raw_json: &str) -> Vec<Country> {
            let items: Vec<Country> =
                serde_json::from_str(raw_json).expect("Failed to parse input JSON");

            items
        }
    }

    pub fn run() {
        println!("Reading file {}", COUNTRIES_JSON);
        let result = self::read_file_as_string();

        match result {
            Ok(content) => {
                self::serialize_json(content.to_string());
                println!("File content {}", content)
            }
            Err(err) => println!("Error when reading file {}", err),
        }
    }

    fn read_file_as_string(path: &str) -> Result<String, io::Error> {
        let mut file = File::open(path)?;
        let mut result = String::new();

        file.read_to_string(&mut result)?;

        Ok(result)
    }

    fn serialize_json(raw_json: &str) -> Vec<Country> {
        let items: Vec<Country> =
            serde_json::from_str(raw_json).expect("Failed to parse input JSON");

        items
    }
}
