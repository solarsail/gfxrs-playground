use glutin;
use glutin::{EventsLoop, Event};
use gfx_window_glutin;
use system::System;
use context::Context;

pub struct SysEventSystem {
    events_loop: EventsLoop,
}

impl SysEventSystem {
    pub fn new(events_loop: EventsLoop) -> SysEventSystem {
        SysEventSystem { events_loop }
    }

    fn update(event: Event, ctx: &mut Context) {
        use glutin::WindowEvent::*;
        use glutin::{MouseScrollDelta, VirtualKeyCode};
        use glutin::ElementState::*;
        if let Event::WindowEvent { event, .. } = event {
            match event {
                Closed => {
                    ctx.running = false; // cannot `break` in closure
                }
                KeyboardInput {
                    input:
                        glutin::KeyboardInput {
                            state,
                            virtual_keycode: Some(vk),
                            ..
                        },
                    ..
                } => match (state, vk) {
                    (_, VirtualKeyCode::Escape) => ctx.running = false,
                    _ => {
                        ctx.key_state.update_key(vk, state == Pressed);
                    }
                },
                MouseMoved {
                    position: (x, y), ..
                } => {
                    ctx.update_mouse_pos(x as i32, y as i32);
                }
                Focused(true) => {
                    ctx.focused();
                }
                MouseEntered { .. } => {
                    ctx.mouse_entered();
                }
                MouseWheel {
                    delta: MouseScrollDelta::LineDelta(_, dy),
                    ..
                } => {
                    ctx.mouse_state.update_scroll(dy);
                }
                Resized(w, h) => {
                    ctx.update_dimensions(w, h);
                    gfx_window_glutin::update_views(
                        &ctx.window,
                        &mut ctx.render_target,
                        &mut ctx.depth_stencil,
                    );
                }
                _ => {}
            }
        }
    }
}

impl System for SysEventSystem {
    fn run(&mut self, ctx: &mut Context, _dt: f32) {
        self.events_loop.poll_events(|event| { SysEventSystem::update(event, ctx); });
    }
}
