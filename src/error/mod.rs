#[derive(Debug)]
pub struct AspenError {
    location: Location,
    err_type: String,
    details: String,
}

impl AspenError {
    pub fn to_string(&self) -> String {
        format!("ERROR at {}:\ntype:     {}\ndetails:  {}", self.location, self.err_type, self.details)
    }
}

impl<T> From<T> for AspenError where 
    T: std::error::Error {
    #[track_caller]
    fn from(value: T) -> Self {
        let here = std::panic::Location::caller();

        AspenError {
            err_type: std::any::type_name::<T>().to_owned(),
            details: value.to_string(),
            location: Location {
                filename: here.file(),
                line: here.line(),
            }
        }
    }
}

impl std::fmt::Display for AspenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct Location {
    pub filename: &'static str,
    pub line: u32,
}

impl Location {
    #[track_caller]
    pub fn here() -> Location {
        let here = std::panic::Location::caller();

        Location {
            filename: here.file(),
            line: here.line(),
        }
    }

    pub fn to_string(&self) -> String {
        format!("line {} in {}", self.line, self.filename)
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}