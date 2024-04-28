//#![allow(unused)]
use winit::{
    event::Event, 
    event_loop::{
        ControlFlow, 
        EventLoop, 
        EventLoopBuilder
    }
};

use crate::{
    renderer::Renderer, 
    timing::TimingStruct, 
    interface::Client
};

#[derive(Debug)]
enum GlobalEvent {
    Update,
    Shutdown,
}

pub struct Application<UD: Client> {
    event_loop: EventLoop<GlobalEvent>,
    user_data: UD,
    timer: TimingStruct,
    renderer: Option<Renderer>,
}

impl<UD: Client> Application<UD> {
    pub fn new(user_data: UD, use_graphics: bool) -> Self {
        let event_loop = EventLoopBuilder::<GlobalEvent>::with_user_event()
            .build()
            .expect("event loop creation failed");

        let renderer = match use_graphics {
            true => Some(Renderer::new()),
            false => None
        };

        Self {
            event_loop,
            user_data,
            timer: TimingStruct::new(),
            renderer,
        }
    }

    pub fn run(mut self) {
        let proxy = self.event_loop.create_proxy();
        self.event_loop.run(move |event, elwt| {
            elwt.set_control_flow(ControlFlow::Poll);
            match event {
                //Event::WindowEvent { event, .. } => match event {
                //    WindowEvent::Resized(size) => {
                //        if size.width != 0 && size.height != 0 {
                //            // Some platforms like EGL require resizing GL surface to update the size
                //            // Notable platforms here are Wayland and macOS, other don't require it
                //            // and the function is no-op, but it's wise to resize it for portability
                //            // reasons.
                //        }
                //    },
                //    WindowEvent::CloseRequested
                //    | WindowEvent::KeyboardInput {
                //        event: KeyEvent { logical_key: Key::Named(NamedKey::Escape), .. },
                //        ..
                //    } => window_target.exit(),
                //    _ => (),
                //},
                Event::AboutToWait => {
                    proxy.send_event(GlobalEvent::Update).unwrap();
                },
                Event::UserEvent(global_event) => {
                    match global_event {
                        GlobalEvent::Update => {
                            let time_info = self.timer.update(100);

                            for _ in 0..time_info.fixed_steps {
                                self.user_data.fixed_update(time_info.fixed_delta);
                            }
                            
                            self.user_data.update(time_info.delta);
                        },
                        GlobalEvent::Shutdown => {
                            elwt.exit()
                        }
                    }
                },
                _ => ()
            }
        }).unwrap()
    }
}