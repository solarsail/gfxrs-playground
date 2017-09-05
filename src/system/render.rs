use context::Context;
use gfx_device_gl::Device;

pub struct RenderSystem {
    device: Device,
}

impl RenderSystem {
    pub fn new(device: Device) -> RenderSystem {
        RenderSystem { device }
    }
    
    pub fn run(&mut self, &mut ctx: Context, dt: f32) {
        for encoder in ctx.encoders.mut_iter() {
            encoder.flush(&mut self.device);
        }
        ctx.window.swap_buffers().unwrap();
        self.device.cleanup();
    }
}