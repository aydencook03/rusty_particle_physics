//! This is a "Minimal-Viable-Product" renderer for the rusty_particle_physics crate.
//! It is very bare bones, and is mainly used to test the actual physics engine.
//!
//! It uses winit for the event_loop and window, std::time for timekeeping, softbuffer
//! for accessing the window's framebuffer, and tiny_skia for the path->pixels (rasterization)
//! algorithms.
//!
//! Key Control:
//! - arrow keys -> move around
//! - plus(equals)/minus -> zoom in/out
//! - space -> pause
//! - q -> quit

use rusty_particle_physics_2d::sim::Sim;
use rusty_particle_physics_2d::vec2::Vec2;

use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent, DeviceEvent, KeyboardInput, VirtualKeyCode, ElementState},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use std::time::Instant;

use softbuffer::GraphicsContext;

use tiny_skia::{Color, FillRule, Paint, PathBuilder, Pixmap, Stroke, Transform};

const DEFAULT_COLOR: (u8, u8, u8) = (70, 70, 70);
const STROKE: f32 = 2.5;
const MOVE_SIZE: f64 = 10.0;

pub struct MinimalRenderer {
    bg_color: (u8, u8, u8),
    view_offset: Vec2,
    event_loop: EventLoop<()>,
    time: Instant,
    context: GraphicsContext<Window>,
}

impl MinimalRenderer {
    /// initialize the renderer with width & height
    pub fn new(width: u16, height: u16) -> Self {
        // A "context" that provides a way to retrieve events from the system and the windows registered to it.
        // EventLoop::new() initializes everything that will be required to create windows.
        // For example on Linux creating an event loop opens a connection to the X or Wayland server.
        let event_loop = EventLoop::new();

        let window = {
            let size = LogicalSize::new(width, height);
            WindowBuilder::new()
                .with_inner_size(size)
                .with_title("Simulation")
                .build(&event_loop)
                .unwrap()
        };

        MinimalRenderer {
            bg_color: DEFAULT_COLOR,
            view_offset: Vec2::new(0.0, 0.0),
            event_loop,
            time: Instant::now(),
            context: unsafe { GraphicsContext::new(window) }.unwrap(),
        }
    }

    /// Converts from an rgb color to the 32-bit format that softbuffer uses.
    ///
    /// Pixel format (u32): 00000000RRRRRRRRGGGGGGGGBBBBBBBB
    pub fn rgb_to_softbuffer(rgb: (u8, u8, u8)) -> u32 {
        let (r, g, b) = rgb;
        let r = (r as u32) << 16;
        let g = (g as u32) << 8;
        let b = (b as u32) << 0;

        r | g | b
    }

    /// a builder method to give the window color upon creation (after calling new)
    pub fn with_color(mut self: Self, color: (u8, u8, u8)) -> Self {
        self.bg_color = color;
        self
    }

    /// a method to dynamically get the window's width.
    ///
    /// this isn't a method and a context needs to be passed in order to avoid
    /// a partial move error. self.event_loop.run consumes self's event loop.
    /// so trying to access self in a method wouldn't work, as a part of self is owned
    /// by the running event loop.
    fn dyn_width(context: &GraphicsContext<Window>) -> f64 {
        context.window().inner_size().width as f64
    }

    /// a method to dynamically get the window's width. see dyn_width.
    fn dyn_height(context: &GraphicsContext<Window>) -> f64 {
        context.window().inner_size().height as f64
    }

    pub fn run(mut self: Self, mut sim: Sim<'static>) {
        self.event_loop.run(move |event, _, control_flow| {
            // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
            // dispatched any events. This is ideal for games and similar applications.
            *control_flow = ControlFlow::Poll;

            let width = Self::dyn_width(&self.context);
            let height = Self::dyn_height(&self.context);

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    // stop the event loop, therefore closing the window
                    *control_flow = ControlFlow::Exit
                }
                Event::DeviceEvent {
                    event: DeviceEvent::Key(KeyboardInput{state: ElementState::Pressed, virtual_keycode: Some(code), ..}),
                    ..
                } => {
                    match code {
                        VirtualKeyCode::Left => self.view_offset.x -= MOVE_SIZE,
                        VirtualKeyCode::Right => self.view_offset.x += MOVE_SIZE,
                        VirtualKeyCode::Up => self.view_offset.y -= MOVE_SIZE,
                        VirtualKeyCode::Down => self.view_offset.y += MOVE_SIZE,
                        VirtualKeyCode::Space => sim.running = !sim.running,
                        VirtualKeyCode::Q => *control_flow = ControlFlow::Exit,
                        _ => (),
                    }
                }
                Event::MainEventsCleared => {
                    // update & render after other events are handled

                    // create the buffers
                    let mut draw_buffer = Pixmap::new(width as u32, height as u32).unwrap();
                    let mut framebuffer: Vec<u32> = Vec::new();

                    // create drawing styles
                    let mut style = Paint::default();
                    style.anti_alias = true;
                    let mut stroke = Stroke::default();
                    stroke.width = STROKE;

                    // paint the background
                    draw_buffer.fill(Color::from_rgba8(
                        self.bg_color.0,
                        self.bg_color.1,
                        self.bg_color.2,
                        255,
                    ));

                    // draw the sim's particles
                    for particle in &sim.particles {
                        let x = (particle.pos.x - self.view_offset.x) as f32;
                        let y = (particle.pos.y - self.view_offset.y) as f32;
                        let radius = particle.radius as f32;
                        let (r, g, b, a) = particle.color;

                        let path = {
                            let mut pb = PathBuilder::new();
                            pb.push_circle(x, y, radius);
                            pb.finish().unwrap()
                        };

                        // draw the particle's outline
                        style.set_color_rgba8(0, 0, 0, 255);
                        draw_buffer.stroke_path(
                            &path,
                            &style,
                            &stroke,
                            Transform::identity(),
                            None,
                        );
                        // fill in the particle's outline
                        style.set_color_rgba8(r, g, b, a);
                        draw_buffer.fill_path(
                            &path,
                            &style,
                            FillRule::Winding,
                            Transform::identity(),
                            None,
                        );
                    }

                    // copy the contents from draw_buffer to framebuffer w/ required format
                    for color in draw_buffer.pixels() {
                        let rgb = (color.red(), color.green(), color.blue());
                        framebuffer.push(MinimalRenderer::rgb_to_softbuffer(rgb));
                    }

                    // write the contents of draw_buffer to the window buffer
                    self.context
                        .set_buffer(&framebuffer, width as u16, height as u16);

                    // find the time since last frame
                    let elapsed = (Instant::now().duration_since(self.time).as_micros() as f64)
                        * (10.0_f64).powi(-6);
                    // step the simulation forward by that time
                    sim.step_simulation(elapsed);

                    // put fps and sim time on window title
                    self.context.window_mut().set_title(
                        format!(
                            "Simulation - fps: {:.0} - time: {:.3}",
                            1.0 / elapsed,
                            sim.time
                        )
                        .as_str(),
                    );
                    // set time for next frame to use
                    self.time = Instant::now();
                }
                _ => (),
            };
        });
    }
}
