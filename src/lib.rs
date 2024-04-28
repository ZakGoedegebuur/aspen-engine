pub mod application;
pub mod interface;
pub mod renderer;
pub mod timing;

/*
use glutin::{config::{Config, ConfigTemplateBuilder}, context::ContextAttributesBuilder, display::GetGlDisplay};
use glutin_winit::{DisplayBuilder, GlWindow};
use glutin::prelude::*;
use winit::{event::{Event, KeyEvent, WindowEvent}, event_loop::EventLoop, keyboard::{Key, NamedKey}, window::WindowBuilder};
use raw_window_handle::HasRawWindowHandle;

fn main() {
    let event_loop = EventLoop::new().expect("failed to create event loop");
    let window_builder = WindowBuilder::new()
        .with_title("cool window");

    let template = ConfigTemplateBuilder::new().with_alpha_size(8).with_transparency(cfg!(cgl_backend));

    let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));

    let (window, gl_config) = display_builder.build(&event_loop, template, gl_config_picker).unwrap();
    let raw_window_handle = window.as_ref().map(|window| window.raw_window_handle());

    let gl_display = gl_config.display();

    let context_attributes = ContextAttributesBuilder::new().build(raw_window_handle);

    let mut not_current_gl_context = Some(unsafe {
        gl_display.create_context(&gl_config, &context_attributes).expect("failed to create gl context")
    });

    let mut window = window.unwrap();

    let attrs = window.build_surface_attributes(Default::default());
    let gl_surface = unsafe {
        gl_config.display().create_window_surface(&gl_config, &attrs).unwrap()
    };

    let gl_context = not_current_gl_context.take().unwrap().make_current(&gl_surface).unwrap();

    event_loop.run(move |event, window_target| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => {
                    if size.width != 0 && size.height != 0 {
                        // Some platforms like EGL require resizing GL surface to update the size
                        // Notable platforms here are Wayland and macOS, other don't require it
                        // and the function is no-op, but it's wise to resize it for portability
                        // reasons.
                    }
                },
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event: KeyEvent { logical_key: Key::Named(NamedKey::Escape), .. },
                    ..
                } => window_target.exit(),
                _ => (),
            },
            _ => ()
        }
    }).unwrap();
}

pub fn gl_config_picker(configs: Box<dyn Iterator<Item = Config> + '_>) -> Config {
    configs
        .reduce(|accum, config| {
            let transparency_check = config.supports_transparency().unwrap_or(false)
                & !accum.supports_transparency().unwrap_or(false);

            if transparency_check || config.num_samples() > accum.num_samples() {
                config
            } else {
                accum
            }
        })
        .unwrap()
}

*/