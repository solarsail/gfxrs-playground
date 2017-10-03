use glutin;
use gfx_window_glutin;
use gfx_device_gl;

use render;
use context::Context;

pub struct App {}

impl App {
    pub fn init(
        title: &str,
        width: u32,
        height: u32,
    ) -> (
        gfx_device_gl::Device,
        gfx_device_gl::Factory,
        glutin::EventsLoop,
        Context,
    ) {
        let events_loop = glutin::EventsLoop::new();
        let context = glutin::ContextBuilder::new();
        let builder = glutin::WindowBuilder::new()
            .with_title(title.to_string())
            .with_dimensions(width, height);

        // gfx-rs init
        let (window, device, factory, render_target, depth_stencil) =
            gfx_window_glutin::init::<render::ColorFormat, render::DepthFormat>(
                builder,
                context,
                &events_loop,
            );

        let context = Context::new(window, width as i32, height as i32, render_target, depth_stencil);
        (device, factory, events_loop, context)
    }
}
