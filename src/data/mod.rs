use csv::StringRecord;
use std::{collections::HashMap, fs::File, io::Write};

use crate::QueryError;

// fixme: eventually the cli params will be passed into here somehow
const CONFIG_FILE_PATH: &str = "./data/config.csv";
const CONFIG_FILE_NAME: &str = "config.csv";
const DATA_DIR_PATH: &str = "./data/";

pub struct DataAccessor {
    pub ready: Vec<String>,
    loaded: HashMap<String, Vec<StringRecord>>,
}

impl DataAccessor {
    /// This is stupid, but I don't have enough braincells to put this into the config right now
    ///
    /// Seems like the smart thing to do would be to make the config a json and have it keep track of the available datasets and their headers
    /// This will do for now so I can throw it into the Parser, this api should be fine
    pub fn get_headers(dataset: &String) -> Result<Vec<String>, QueryError> {
        let path = DATA_DIR_PATH.to_string() + dataset.as_ref() + ".csv";
        let mut reader = csv::Reader::from_path(path)?;
        let headers = reader
            .headers()?
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        Ok(headers)
    }

    pub fn get(&mut self, dataset: String) -> Result<&Vec<StringRecord>, QueryError> {
        if !self.loaded.contains_key(&dataset) {
            self.load(dataset.clone())?;
        }
        Ok(&self.loaded.get(&dataset).unwrap())
    }

    fn load(&mut self, dataset: String) -> Result<(), QueryError> {
        if !self.ready.contains(&dataset) {
            Err(QueryError::BadSyntax("Requested dataset does not exist"))
        } else {
            let path = DATA_DIR_PATH.to_string() + dataset.as_ref() + ".csv";
            let mut reader = csv::Reader::from_path(path)?;
            let loaded: Result<Vec<StringRecord>, QueryError> = reader
                .records()
                // Convert csv errors to QueryErrors
                .map(|r| r.map_err(|e| QueryError::from(e)))
                .collect();
            self.loaded.insert(dataset, loaded?);
            Ok(())
        }
    }

    /// Attempt to read the available datasets from the config.csv file
    /// Errors if the config file doesnt exist or is invalid csv. In both of those cases, we should recreate the config file
    fn read_config() -> Result<Vec<String>, QueryError> {
        let mut config = csv::Reader::from_path(CONFIG_FILE_PATH)?;
        // for x in config.records() {
        //     println!("{:?}", x);
        // }
        config
            .records()
            .map(|str_record_res| match str_record_res {
                Ok(str_record) => Ok(str_record[0].to_string()),
                Err(e) => Err(QueryError::from(e)),
            })
            .collect()
    }

    /// Called when the config file doesn't exist/can't be accessed anymore: so create a new config file
    /// Returns the list of dataset filenames loaded from the data dir
    fn recreate_config() -> Result<Vec<String>, QueryError> {
        println!(">> Recreating config");
        let mut config_file = File::create(CONFIG_FILE_PATH)?;
        let mut datasets = Vec::new();
        config_file.write_all(b"dataset\n")?;

        for dataset_name in std::fs::read_dir(DATA_DIR_PATH)? {
            let name = dataset_name?.file_name().into_string();
            match name {
                Ok(name) => {
                    // Filter the config from the available datasets
                    if name.as_str() == CONFIG_FILE_NAME {
                        continue;
                    } else {
                        // Remove the .csv file extension
                        let cleaned = name.split(".csv").collect::<Vec<&str>>()[0];
                        config_file.write_all(cleaned.as_bytes())?;
                        config_file.write_all("\n".as_bytes())?;
                        datasets.push(cleaned.to_string());
                    }
                }
                Err(_) => {
                    return Err(QueryError::FileError(String::from(
                        "Invalid unicode data in filename, unable to use as dataset name",
                    )));
                }
            }
        }
        Ok(datasets)
    }

    /// Try to load config data to prepare for reading data
    ///
    /// May fail if there are invalid dataset names or the os is unable to read/write to the data dir/config file
    pub fn new() -> Result<DataAccessor, QueryError> {
        let ready = match DataAccessor::read_config() {
            Ok(datasets) => datasets,
            Err(_) => DataAccessor::recreate_config()?, // Old config file was invalid for some reason, rewrite it
        };
        Ok(DataAccessor {
            ready,
            loaded: HashMap::new(),
        })
    }
}

impl From<std::io::Error> for QueryError {
    fn from(e: std::io::Error) -> Self {
        QueryError::FileError(e.to_string())
    }
}

impl From<csv::Error> for QueryError {
    fn from(e: csv::Error) -> Self {
        QueryError::BadCSV(e.to_string())
    }
}
