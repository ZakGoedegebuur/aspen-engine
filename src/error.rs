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