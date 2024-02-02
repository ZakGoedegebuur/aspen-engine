use std::{error::Error, io::Write};

pub struct AspenLogger {
    logs: Vec<String>,
    file: std::fs::File,
}

impl AspenLogger {
    pub fn new(output_filepath: String) -> Result<AspenLogger, Box<dyn Error>> {
        let prefix = std::path::Path::new(output_filepath.as_str())
            .parent()
            .unwrap_or(std::path::Path::new("/"));

        std::fs::create_dir_all(prefix)?;

        let file = std::fs::File::create(output_filepath.clone())?;

        Ok(AspenLogger {
            logs: Vec::new(),
            file,
        })
    }

    pub fn log(&mut self, error: impl std::string::ToString) {
        self.logs.push(error.to_string())
    }

    fn write_all(&mut self) {
        let serialised = serde_json::to_string_pretty(&self.logs)
            .expect("failed to serialise logs");
        self.file.write(serialised.as_bytes()).expect("failed to write logs to file");
    }
}

impl Drop for AspenLogger {
    fn drop(&mut self) {
        self.write_all()
    }
}