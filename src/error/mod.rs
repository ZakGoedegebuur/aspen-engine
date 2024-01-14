#[derive(Debug, Clone)]
pub struct Location {
    pub filename: &'static str,
    pub line: u32,
    pub column: u32,
}

impl Location {
    #[track_caller]
    pub fn here() -> Location {
        let here = std::panic::Location::caller();

        Location {
            filename: here.file(),
            line: here.line(),
            column: here.column(),
        }
    }

    pub fn as_string(&self) -> String {
        format!("{}:{} in {}", self.line, self.column, self.filename)
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

pub trait Error {
    fn get_message(&self) -> String;
    fn get_details(&self) -> String;
    fn get_location(&self) -> Location;
}
