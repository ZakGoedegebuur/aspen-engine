use std::{collections::VecDeque, io::Write};

use crate::error::AspenError;

pub struct Logger {
    logs: VecDeque<AspenError>,
    file: std::fs::File,
}

impl Logger {
    pub fn new(output_filepath: String) -> Result<Logger, AspenError> {
        let file = match std::fs::File::create(output_filepath.clone()) {
            Ok(val) => val,
            Err(err) => return Err(
                AspenError::new(
                    "FileCreationError".to_owned(), 
                    err.to_string(), 
                    crate::error::AspenErrorSeverity::Error
                )
            )
        };

        Ok(Logger {
            logs: VecDeque::new(),
            file,
        })
    }

    pub fn log(&mut self, error: AspenError) {
        self.logs.push_front(error);
    }

    fn write_all(&mut self) {
        let msg_arr = self.logs.make_contiguous();
        let serialised = serde_json::to_string(&msg_arr).expect("failed to serialise logs");
        self.file.write(serialised.as_bytes()).expect("failed to write logs to file");
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        self.write_all()
    }
}