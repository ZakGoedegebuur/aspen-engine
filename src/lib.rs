use winit::event_loop::EventLoop;

pub mod error;
pub mod graphics;

use error::{
    Error,
    Location
};

pub struct Framework<PT> {
    event_loop: EventLoop<()>,
    windows: Vec<()>,
    persistent_data: PT,
}

impl<PT> Framework<PT> {
    pub fn new(persistent_data: PT) -> Result<Framework<PT>, FrameworkInitError> {

        let event_loop = match EventLoop::new() {
            Ok(eloop) => eloop,
            Err(err) => return Err(
                FrameworkInitError {
                    location: error::Location::here(),
                    message: "Event loop initialisation failed".to_owned(),
                    details: err.to_string()
                }
            )
        };

        Ok(Framework {
            event_loop,
            windows: vec![],
            persistent_data,
        })
    }

    pub fn run(self) {

    }
}

pub struct FrameworkInitError {
    location: Location,
    message: String,
    details: String,
}

impl Error for FrameworkInitError {
    fn get_message(&self) -> String {
        self.message.clone()
    }

    fn get_details(&self) -> String {
        self.details.clone()
    }

    fn get_location(&self) -> Location {
        self.location.clone()
    }
}