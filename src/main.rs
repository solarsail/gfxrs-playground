extern crate cgmath;
extern crate find_folder;
#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate image;
#[macro_use]
extern crate lazy_static;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use std::time;
use gfx::Device;
use glutin::GlContext;
use cgmath::{Matrix4, Point3, Vector3};
use cgmath::prelude::*;

mod render;
mod model;
mod camera;
mod context;
mod system;

use system::CameraSystem;
use context::Context;
use camera::CameraBuilder;


const SCREEN_WIDTH: i32 = 1024;
const SCREEN_HEIGHT: i32 = 768;

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let context = glutin::ContextBuilder::new();
    let builder = glutin::WindowBuilder::new()
        .with_title("Learn OpenGL".to_string())
        .with_dimensions(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);

    // gfx-rs init
    let (window, mut device, mut factory, mut render_target, mut depth_stencil) =
        gfx_window_glutin::init::<render::ColorFormat, render::DepthFormat>(
            builder,
            context,
            &events_loop,
        );
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let material = render::Material::new(
        &mut factory,
        "textures/container2.png",
        "textures/container2_specular.png",
        32.0,
    );
    let mut cubes = Vec::new();
    for pos in model::cube_positions().into_iter() {
        cubes.push(render::Object::new(&mut factory, model::vertices(), Matrix4::from_translation(pos), material.clone()));
    }
    let cube = render::Object::new(
        &mut factory,
        model::vertices(),
        Matrix4::identity(),
        material,
    );
    let cube_brush = render::ObjectBrush::new(&mut factory);

    // positions of the point lights
    let point_light_positions = vec![
        Vector3::new( 0.7,  0.2,  2.0),
        Vector3::new( 2.3, -3.3, -4.0),
        Vector3::new(-4.0,  2.0, -12.0),
        Vector3::new( 0.0,  0.0, -3.0)
    ];

    let light_dir = Vector3::new(1.2, 1.0, 2.0);
    let light_color = Vector3::new(1.0, 1.0, 1.0);
    let trans = Matrix4::from_translation(light_dir);
    let scale = Matrix4::from_scale(0.2);
    let light_model = trans * scale;

    let dir_light = render::DirLight::new(
        (light_color * 0.1),
        (light_color * 0.5),
        light_color,
        light_dir,
    );

    let mut point_lights = Vec::new();
    for pos in point_light_positions.iter() {
        point_lights.push(render::PointLight::new(light_color * 0.1, light_color * 0.5, light_color, pos.clone()));
    }

    // Game loop
    //let start_time = time::Instant::now();
    let mut last_frame = time::Instant::now();
    let mut running = true;
    let camera = CameraBuilder::new(Point3::new(0.0, 0.0, 3.0), Vector3::unit_y())
        .aspect(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32)
        .build();


    let mut context = Context::new(&window, SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut cs = CameraSystem::new(camera, 0.1);

    while running {
        let current_frame = time::Instant::now();
        let dt = current_frame.duration_since(last_frame);
        //let elapsed = current_frame.duration_since(start_time);
        last_frame = current_frame;
        events_loop.poll_events(|event| {
            use glutin::WindowEvent::*;
            use glutin::{MouseScrollDelta, VirtualKeyCode};
            use glutin::ElementState::*;
            if let glutin::Event::WindowEvent { event, .. } = event {
                match event {
                    Closed => {
                        running = false; // cannot `break` in closure
                    }
                    KeyboardInput {
                        input: glutin::KeyboardInput {
                            state,
                            virtual_keycode: Some(vk),
                            ..
                        },
                        ..
                    } => match (state, vk) {
                        (_, VirtualKeyCode::Escape) => running = false,
                        _ => {
                            context.key_state.update_key(vk, state == Pressed);
                        }
                    },
                    MouseMoved {
                        position: (x, y), ..
                    } => {
                        context.update_mouse_pos(x as i32, y as i32);
                    }
                    Focused(true) => {
                        context.focused();
                    }
                    MouseEntered { .. } => {
                        context.mouse_entered();
                    }
                    MouseWheel {
                        delta: MouseScrollDelta::LineDelta(_, dy),
                        ..
                    } => {
                        context.mouse_state.update_scroll(dy);
                    }
                    Resized(w, h) => {
                        context.update_dimensions(w, h);
                        gfx_window_glutin::update_views(
                            &window,
                            &mut render_target,
                            &mut depth_stencil,
                        );
                    }
                    _ => {}
                }
            }
        });

        let dt = dt.as_secs() as f32 + dt.subsec_nanos() as f32 / 1e9;
        //let elapsed = elapsed.as_secs() as f32 + elapsed.subsec_nanos() as f32 / 1e9;
        cs.run(&mut context, dt);


        let camera = cs.camera();
        encoder.clear(&render_target, render::BLACK);
        encoder.clear_depth(&depth_stencil, 1.0);
        cube_brush.draw(
            &cube,
            &dir_light,
            &point_lights,
            camera,
            &render_target,
            &depth_stencil,
            &mut encoder,
        );
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
