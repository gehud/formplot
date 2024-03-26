use std::{error::Error, path::{Path, PathBuf}};

use csv::Reader;
use formplot_core::Fetch;

pub struct CSV {
    path: PathBuf
}

impl CSV {
    pub fn from<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf()
        }
    }
}

impl Fetch for CSV {
    fn fetch(&self) -> Result<Vec<(String, Vec<String>)>, Box<dyn Error>> {
        let mut result = Vec::<(String, Vec::<String>)>::new();
        
        let mut reader = Reader::from_path(&self.path).unwrap();

        let headers = reader.headers()?;
        for header in headers {
            result.push((String::from(header), Vec::new()));
        }

        let records = reader.records();
        for item in records {
            let record = item?;
            for column in 0..result.len() {
                let value = record.get(column).unwrap();
                result[column].1.push(String::from(value));
            }
        }

        Ok(result)
    }
}