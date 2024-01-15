use winit::event_loop::EventLoop;

pub mod error;
pub mod graphics;

use error::AspenError;

pub struct Engine<PT> {
    event_loop: EventLoop<()>,
    windows: Vec<()>,
    persistent_data: PT,
}

impl<PT> Engine<PT> {
    pub fn new(persistent_data: PT) -> Result<Engine<PT>, AspenError> {
        let event_loop = EventLoop::new()?;

        let contents = std::fs::read_to_string("fake_path.txt")?;

        Ok(Engine {
            event_loop,
            windows: vec![],
            persistent_data,
        })
    }

    pub fn run(self) {

    }
}
