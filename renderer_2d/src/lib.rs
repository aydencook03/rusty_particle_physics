//! This is an example implementation renderer for the
//! [`rusty_particle_physics_2d`](https://github.com/aydencook03/rusty_particle_physics) crate.
//! It is pretty simple, and is mainly used to test the 2d physics engine.
//!
//! It uses [`winit`](https://github.com/rust-windowing/winit) for the event_loop, window, and keyboard,
//! [`std::time`](https://doc.rust-lang.org/std/time/index.html) for timekeeping,
//! [`softbuffer`](https://github.com/john01dav/softbuffer) for accessing the window's framebuffer,
//! and [`tiny_skia`](https://github.com/RazrFalcon/tiny-skia) for turning shapes into
//! pixels (the rasterization algorithms).
//!
//!
//! # Controls:
//!
//! |  Key   |   Action    |
//! |--------|-------------|
//! | Arrows | Pan Around  |
//! | +/-    | Zoom In/Out |
//! | Enter  | Reset View  |
//! | Space  | Pause/Play  |
//! | R      | Reset Sim   |
//! | Q      | Quit        |
//!
//!
//! # Example usage:
//!
//! ```rust
//! use rusty_particle_physics_2d::prelude::*;
//! use renderer_2d::Renderer;
//!
//! let window = Renderer::new(600, 600);
//! let mut sim = Sim::new();
//!
//! // Set up the simulation... //
//!
//! window.run(sim);
//! ```

use rusty_particle_physics_2d::interaction;
use rusty_particle_physics_2d::rendering::View2D;
use rusty_particle_physics_2d::sim::Sim;
use rusty_particle_physics_2d::vec2::Vec2;

use winit::{
    dpi::PhysicalSize, //LogicalSize,
    event::{DeviceEvent, ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use std::time::Instant;

use softbuffer::GraphicsContext;

use tiny_skia::{Color, FillRule, Paint, PathBuilder, Pixmap, Stroke, Transform};

const STROKE: f32 = 2.5;
const STROKE_COLOR: (u8, u8, u8, u8) = rusty_particle_physics_2d::particle::BLACK;

pub struct Renderer {
    view: View2D,
    event_loop: EventLoop<()>,
    context: GraphicsContext<Window>,
}

impl Renderer {
    /// Initialize the renderer with width & height.
    pub fn new(width: u32, height: u32) -> Self {
        // A "context" that provides a way to retrieve events from the system and the windows registered to it.
        // EventLoop::new() initializes everything that will be required to create windows.
        // On Linux, creating an event loop opens a connection to the X or Wayland server.
        let event_loop = EventLoop::new();

        let window = {
            let size = PhysicalSize::new(width, height);
            WindowBuilder::new()
                .with_inner_size(size)
                .with_title("Simulation")
                .build(&event_loop)
                .unwrap()
        };

        Renderer {
            view: View2D::new(),
            event_loop,
            context: unsafe { GraphicsContext::new(window) }.unwrap(),
        }
    }

    /// A builder method to give the window a non-default color upon creation (after calling new). Ex:
    ///
    /// ```rust
    /// let window = Renderer::new(WIDTH, HEIGHT).with_color(((R, G, B, A)));
    /// ```
    pub fn with_color(mut self: Self, color: (u8, u8, u8, u8)) -> Self {
        self.view.bg_color = color;
        self
    }

    /// Converts from an rgb color to the 32-bit binary format that softbuffer uses.
    ///
    /// Pixel format (u32): 00000000RRRRRRRRGGGGGGGGBBBBBBBB
    fn rgb_to_softbuffer(rgb: (u8, u8, u8)) -> u32 {
        let (r, g, b) = rgb;
        let r = (r as u32) << 16;
        let g = (g as u32) << 8;
        let b = (b as u32) << 0;

        r | g | b
    }

    /// A method to dynamically get the window's width.
    ///
    /// This isn't a method and a context needs to be passed in order to avoid
    /// a partial move error. self.event_loop.run consumes self's event loop.
    /// so trying to access self in a method wouldn't work, as a part of self is owned
    /// by the running event loop.
    fn dyn_width(context: &GraphicsContext<Window>) -> u32 {
        context.window().inner_size().width
    }

    /// A method to dynamically get the window's width. see dyn_width.
    fn dyn_height(context: &GraphicsContext<Window>) -> u32 {
        context.window().inner_size().height
    }

    /// Run the given simulation in a new window.
    pub fn run(mut self: Self, mut sim: Sim<'static>) {
        let mut time = Instant::now();

        self.event_loop.run(move |event, _, control_flow| {
            // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
            // dispatched any events. This is ideal for games and similar applications.
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    // stop the event loop, and therefore close the window
                    *control_flow = ControlFlow::Exit
                }
                Event::DeviceEvent {
                    event:
                        DeviceEvent::Key(KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(code),
                            ..
                        }),
                    ..
                } => match code {
                    VirtualKeyCode::Left => self.view.pan_left(),
                    VirtualKeyCode::Right => self.view.pan_right(),
                    VirtualKeyCode::Up => self.view.pan_up(),
                    VirtualKeyCode::Down => self.view.pan_down(),
                    VirtualKeyCode::Equals => self.view.zoom_in(),
                    VirtualKeyCode::Minus => self.view.zoom_out(),
                    VirtualKeyCode::Return => self.view.reset(),
                    VirtualKeyCode::Space => interaction::pause_play(&mut sim),
                    VirtualKeyCode::R => interaction::restart(&mut sim),
                    VirtualKeyCode::Q => *control_flow = ControlFlow::Exit,
                    _ => (),
                },
                Event::MainEventsCleared => {
                    // update & render after other events are handled

                    // get window width, height, and zoom
                    let width = Self::dyn_width(&self.context) as f64;
                    let height = Self::dyn_height(&self.context) as f64;

                    // create buffers
                    let mut draw_buffer = Pixmap::new(width as u32, height as u32).unwrap();
                    let mut framebuffer: Vec<u32> = Vec::new();

                    // create drawing styles
                    let mut style = Paint::default();
                    style.anti_alias = true;
                    let mut stroke = Stroke::default();
                    stroke.width = STROKE * (self.view.parameterized_zoom() as f32);

                    // paint the background
                    draw_buffer.fill(Color::from_rgba8(
                        self.view.bg_color.0,
                        self.view.bg_color.1,
                        self.view.bg_color.2,
                        self.view.bg_color.3,
                    ));

                    // draw the sim's particles
                    for particle in &sim.particles {
                        // get particle position and radius mapped to window space
                        let (Vec2 { x, y }, radius) =
                            self.view.map_to_view(particle.pos, particle.radius);
                        let (r, g, b, a) = particle.color;

                        let path = {
                            let mut pb = PathBuilder::new();
                            // draw the particle using (0,0) to be in the center of the screen
                            pb.push_circle(
                                (x + width / 2.0) as f32,
                                (height / 2.0 - y) as f32,
                                radius as f32,
                            );
                            pb.finish().unwrap()
                        };

                        // draw the particle outlines
                        style.set_color_rgba8(
                            STROKE_COLOR.0,
                            STROKE_COLOR.1,
                            STROKE_COLOR.2,
                            STROKE_COLOR.3,
                        );
                        draw_buffer.stroke_path(
                            &path,
                            &style,
                            &stroke,
                            Transform::identity(),
                            None,
                        );

                        // fill in the particle outlines
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
                    for pixel in draw_buffer.pixels() {
                        let rgb = (pixel.red(), pixel.green(), pixel.blue());
                        framebuffer.push(Renderer::rgb_to_softbuffer(rgb));
                    }

                    // write the contents of framebuffer to the window buffer
                    self.context
                        .set_buffer(&framebuffer, width as u16, height as u16);

                    // find the time since last frame
                    let elapsed = (Instant::now().duration_since(time).as_micros() as f64)
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
                    time = Instant::now();
                }
                _ => (),
            };
        });
    }

    /// Render/Bake an animation instead of running real-time in a window.
    pub fn create_animation(self: &Self, _sim: Sim, _fps: u8, _length: f64) {
        todo!();
    }
}
