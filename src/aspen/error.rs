#[derive(Debug)]
pub struct Error {
    pub error_type: ErrorType,
    pub message: String,
}

impl Error {
    pub fn new(error_type: ErrorType, message: String) -> Error {
        Error {
            error_type,
            message,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}\n", self.message)
    }
}

impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }

    fn description(&self) -> &str {
        &self.message
    }

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub enum ErrorType {
    // Renderer errors
    VulkanMissing,
    EventLoopCreationFailed,
    VulkanInstanceCreationFailed,
    WindowCreationFailed,
    VulkanSurfaceCreationFailed,
}

// panics if msgbox creation fails
pub fn crash_notif(err: Error) {
    match msgbox::create(
        "Crash", 
        format!("Error {:?}:\n{}", err.error_type, err.message).as_str(), 
        msgbox::IconType::Error
    ) {
        Ok(_) => return,
        Err(msgbox_err) => {
            panic!("message box creation error: {}\ninternal error was: {}", msgbox_err.to_string(), err.message);
        }
    }
}