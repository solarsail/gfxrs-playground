use context::Context;

pub mod camera;
pub mod sysevent;

pub trait System {
    fn run(&mut self, ctx: &mut Context, dt: f32);
}

pub use self::camera::CameraSystem;
pub use self::sysevent::SysEventSystem;
