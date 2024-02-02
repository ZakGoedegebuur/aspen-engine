use std::error::Error;

use graphics::Graphics;
use winit::{event_loop::{EventLoop, ControlFlow}, event::{Event, WindowEvent}};

pub mod graphics;
pub mod logging;

use logging::AspenLogger;

pub struct Engine {
    event_loop: EventLoop<()>,
    graphics: Option<Graphics>,
    logger: AspenLogger,
}

impl Engine {
    pub fn new(log_filepath: String) -> Result<Engine, Box<dyn Error>> {
        let mut logger = AspenLogger::new(log_filepath)?;

        let event_loop = EventLoop::new().map_err(|err| { logger.log(&err); err})?;

        Ok(Engine {
            event_loop,
            graphics: None,
            logger,
        })
    }

    pub fn use_graphics(&mut self) -> Result<(), Box<dyn Error>> {
        self.graphics = Some(Graphics::new(&mut self.logger, &self.event_loop)?);
        Ok(())
    }

    pub fn open_window(&mut self) -> Result<(), Box<dyn Error>> {
        match self.graphics {
            Some(ref mut graphics) => graphics.add_window(&self.event_loop)
                .map_err(|err| { self.logger.log(&err); err })?,
            None => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other, "tried to add window when graphics object is not present. Call .use_graphics() on the engine object before initialising any windows. One window will be provided for you when you call .use_graphics()"
                )))
            }
        }

        Ok(())
    }

    pub fn run(mut self) -> Result<(), Box<dyn Error>> {
        self.event_loop.run(move |event, elwt| {
            elwt.set_control_flow(ControlFlow::Poll);
            match event {
                Event::WindowEvent { window_id, event } => {
                    match event {
                        WindowEvent::CloseRequested => _ = {
                            let windows = &mut self.graphics.as_mut().unwrap().windows;
                            if let Some(window_ind) = windows.iter().position(
                                |window| window.id() == window_id
                            ) {
                                if window_ind == 0 {
                                    windows.clear();
                                    elwt.exit();
                                } else {
                                    windows.remove(window_ind);
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
        })?;

        Ok(())
    }
}
