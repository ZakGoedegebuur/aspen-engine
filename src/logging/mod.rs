use std::{collections::VecDeque, io::Write};

use crate::error::{
    AspenError,
    AspenErrorSeverity
};

pub struct AspenLogger {
    logs: VecDeque<AspenError>,
    file: std::fs::File,
}

impl AspenLogger {
    pub fn new(output_filepath: String) -> Result<AspenLogger, AspenError> {
        let prefix = std::path::Path::new(output_filepath.as_str())
            .parent()
            .unwrap_or(std::path::Path::new("/"));

        match std::fs::create_dir_all(prefix) {
            Ok(_) => (),
            Err(err) => return Err(
                AspenError::new(
                    "Failed to create path".to_owned(), 
                    err.to_string(), 
                    AspenErrorSeverity::LoggingError
                )
            )
        }

        let file = match std::fs::File::create(output_filepath.clone()) {
            Ok(val) => val,
            Err(err) => return Err(
                AspenError::new(
                    "FileCreationError".to_owned(),
                    err.to_string(), 
                    AspenErrorSeverity::LoggingError
                )
            )
        };

        Ok(AspenLogger {
            logs: VecDeque::new(),
            file,
        })
    }

    pub fn log(&mut self, error: AspenError) -> AspenError {
        self.logs.push_front(error.clone());
        error
    }

    fn write_all(&mut self) {
        let msg_arr = self.logs.make_contiguous();
        let serialised = serde_json::to_string_pretty(&msg_arr).expect("failed to serialise logs");
        self.file.write(serialised.as_bytes()).expect("failed to write logs to file");
    }
}

impl Drop for AspenLogger {
    fn drop(&mut self) {
        self.write_all()
    }
}