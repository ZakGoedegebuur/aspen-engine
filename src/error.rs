#[derive(Debug)]
pub struct Error {
    pub error_type: ErrorType,
    pub message: String,
    pub description: String,
}

impl Error {
    pub fn new(error_type: ErrorType, message: String, description: String) -> Error {
        Error {
            error_type,
            message,
            description,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}\nDesc: {}", self.message, self.description)
    }
}

impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub enum ErrorType {
    // Renderer errors
    VulkanMissing,
    WinitEventLoopCreationFailed,
    FailedToCreateVulkanInstance,
    WinitWindowCreationFailed,
    SurfaceCreationFailed,
}