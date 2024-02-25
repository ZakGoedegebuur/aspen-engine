use std::{
    error::Error, 
    sync::{
        Arc, 
        Mutex
    }, 
    time::SystemTime
};

use graphics::{
    window::AspenWindow, 
    Graphics
};

use winit::{
    event::{
        Event, 
        WindowEvent
    }, 
    event_loop::{
        ControlFlow, EventLoop, EventLoopBuilder
    }, 
    window::WindowId
};

pub mod graphics;
pub mod logging;

use logging::AspenLogger;

pub struct AppBuilder {
    logger: AspenLogger,
    event_loop: EventLoop<GlobalUpdateEvent>,
    graphics: Option<Graphics>,
    update_funcs: Vec<fn(&mut Application)>
}
#[derive(Debug)]
struct GlobalUpdateEvent;
impl AppBuilder {
    pub fn new() -> Result<AppBuilder, Box<dyn Error>> {
        let mut logger = AspenLogger::new("log/log.json".to_string())?;
        let event_loop = EventLoopBuilder::<GlobalUpdateEvent>::with_user_event()
            .build()
            .map_err(|err| { logger.log(&err); err})?;
        Ok(AppBuilder {
            logger,
            event_loop,
            graphics: None,
            update_funcs: Vec::new(),
        })
    }

    pub fn use_graphics(&mut self) -> Result<(), Box<dyn Error>> {
        self.graphics = Some(
            Graphics::new(&mut self.logger, &self.event_loop)
                .map_err(|err| { self.logger.log(&err); err })?
        );
        Ok(())
    }

    pub fn add_update_func(&mut self, func: fn(&mut Application)) {
        self.update_funcs.push(func)
    }

    pub fn build(self) -> Result<Engine, Box<dyn Error>> {
        Ok(Engine {
            event_loop: self.event_loop,
            application: Application {
                graphics: self.graphics,
                ecs: hecs::World::new(),
            },
            logger: self.logger,
            begin_time: SystemTime::now(),
            update_funcs: self.update_funcs,
        })
    }
}

pub struct Application {
    pub graphics: Option<Graphics>,
    pub ecs: hecs::World,
}

impl Application {
    fn window_from_id(&mut self, id: &WindowId) -> Option<&Arc<Mutex<AspenWindow>>> {
        let ind = self.window_ind_from_id(id)?;
        Some(&self.graphics.as_mut()?.windows[ind])
    }
    
    fn window_ind_from_id(&self, id: &WindowId) -> Option<usize> {
        let graphics = self.graphics.as_ref()?;
        graphics.windows
        .iter()
        .position(|window| window.lock().unwrap().id() == *id)
    }
}

pub struct Engine {
    event_loop: EventLoop<GlobalUpdateEvent>,
    application: Application,
    logger: AspenLogger,
    begin_time: SystemTime,
    update_funcs: Vec<fn(&mut Application)>,
}

impl Engine {
    pub fn use_graphics(&mut self) -> Result<(), Box<dyn Error>> {
        self.application.graphics = Some(
            Graphics::new(&mut self.logger, &self.event_loop)
                .map_err(|err| { self.logger.log(&err); err })?
        );
        Ok(())
    }

    pub fn open_window(&mut self) -> Result<(), Box<dyn Error>> {
        match self.application.graphics {
            Some(ref mut graphics) => graphics.add_window(&self.event_loop)
                .map_err(|err| { self.logger.log(&err); err })?,
            None => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other, "Tried to add window when graphics object is not present. Call .use_graphics() on the engine object before initialising any windows. One window will be provided for you when you call .use_graphics()"
                )))
            }
        }

        Ok(())
    }

    pub fn run(mut self) -> Result<(), Box<dyn Error>> {
        let proxy = self.event_loop.create_proxy();
        self.event_loop.run(|event, elwt| {
            elwt.set_control_flow(ControlFlow::Poll);
            match event {
                Event::WindowEvent { window_id, event } => {
                    match event {
                        WindowEvent::CloseRequested => _ = {
                            let windows = &mut self.application.graphics.as_mut().unwrap().windows;
                            if let Some(window_ind) = windows.iter().position(
                                |window| window.lock().unwrap().id() == window_id
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
                            let window = self.application.window_from_id(&window_id).unwrap();
                            window.lock().unwrap().set_recreate_swapchain();
                        },
                        WindowEvent::RedrawRequested => {
                            let application = &mut self.application;
                            let index = application.window_ind_from_id(&window_id).unwrap();
                            let graphics = application.graphics.as_mut().unwrap();
                            let window = &mut graphics.windows[index];
                            if window.lock().unwrap().should_recreate_swapchain {
                                window
                                    .lock()
                                    .unwrap()
                                    .recreate_swapchain(&graphics.vk_device)
                                    .expect("swapchain recreation failed");

                                println!("recreating swapchain {:#?}", self.begin_time.elapsed().unwrap().as_millis())
                            } 

                            proxy.send_event(GlobalUpdateEvent).unwrap();
                        }
                        _ => ()
                    }
                },
                Event::AboutToWait => {
                    proxy.send_event(GlobalUpdateEvent).unwrap();
                },
                Event::UserEvent(GlobalUpdateEvent) => {
                    for func in self.update_funcs.iter() {
                        (func)(&mut self.application)
                    }
                }
                _ => ()
            }
        })?;

        Ok(())
    }
}
