use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct AspenError {
    location: Location,
    message: String,
    details: String,
    severity: AspenErrorSeverity
}

impl AspenError {
    pub fn to_string(&self) -> String {
        format!("ERROR at {}:\ntype:     {}\ndetails:  {}", self.location, self.message, self.details)
    }
}

impl AspenError {
    #[track_caller]
    pub fn new(message: String, details: String, severity: AspenErrorSeverity) -> AspenError {
        let here = std::panic::Location::caller();
        AspenError {
            location: Location {
                filename: here.file(),
                line: here.line(),
            },
            message,
            details,
            severity
        }
    }
}

impl<T> From<T> for AspenError where 
    T: std::error::Error {
    #[track_caller]
    fn from(value: T) -> Self {
        let here = std::panic::Location::caller();

        AspenError {
            message: std::any::type_name::<T>().to_owned(),
            details: value.to_string(),
            location: Location {
                filename: here.file(),
                line: here.line(),
            },
            severity: AspenErrorSeverity::Unknown
        }
    }
}

impl std::fmt::Display for AspenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Serialize)]
pub enum AspenErrorSeverity {
    Error,
    Warn,
    Info,
    Unknown
}

#[derive(Debug, Clone, Serialize)]
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