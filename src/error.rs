#[derive(Debug)]
pub struct Error {
    error_type: ErrorType,
    message: String,
    description: String,
}

impl Error {
    pub fn new(error_type: ErrorType, message: String, description: String) -> Error {
        Error {
            error_type,
            message,
            description
        }
    }
}

#[derive(Debug)]
pub enum ErrorType {
    // Renderer errors
    VulkanMissing,
    WinitEventLoopCreationFailed,
    FailedToCreateVulkanInstance,
}