use graphics::window::AspenWindow;
use winit::{event_loop::{EventLoop, ControlFlow}, event::{Event, WindowEvent}};

pub mod error;
pub mod graphics;
pub mod logging;

use error::AspenError;

pub struct Engine<PT> {
    event_loop: EventLoop<()>,
    windows: Vec<AspenWindow>,
    persistent_data: PT,
}

impl<PT> Engine<PT> {
    pub fn new(persistent_data: PT) -> Result<Engine<PT>, AspenError> {
        let event_loop = EventLoop::new()?;

        Ok(Engine {
            event_loop,
            windows: vec![],
            persistent_data,
        })
    }

    pub fn open_window(&mut self) -> Result<(), AspenError> {
        let new_window = AspenWindow::new(&self.event_loop)?;

        self.windows.push(new_window);
        Ok(())
    }

    pub fn run(mut self) {
        let _ = self.event_loop.run(move |event, elwt| {
            elwt.set_control_flow(ControlFlow::Poll);
            match event {
                Event::WindowEvent { window_id, event } => {
                    match event {
                        WindowEvent::CloseRequested => _ = {
                            if let Some(window_ind) = self.windows.iter().position(
                                |window| window.id() == window_id
                            ) {
                                if window_ind == 0 {
                                    self.windows.clear();
                                    elwt.exit();
                                } else {
                                    self.windows.remove(window_ind);
                                }
                            }
                        },
                        WindowEvent::Resized(_) => {
                            //self.windows[0].recreate_swapchain = true;
                        },
                        WindowEvent::RedrawRequested => {
                        }
                        _ => ()
                    }

                },
                Event::AboutToWait => {
                    //println!("waiting")
                },
                _ => ()
            }
        });
    }
}
