//! This is a "Minimal Viable Product" renderer for the `rusty_particle_physics` crate.
//! It is very bare bones, and is mainly used to test the actual physics engine.
//!
//! It uses [winit](https://github.com/rust-windowing/winit) for the event_loop, window, and keyboard,
//! [std::time](https://doc.rust-lang.org/std/time/index.html) for timekeeping,
//! [softbuffer](https://github.com/john01dav/softbuffer) for accessing the window's framebuffer,
//! and [tiny_skia](https://github.com/RazrFalcon/tiny-skia) for the path -> pixels (rasterization)
//! algorithms.
//!
//! Key Controls:
//!
//! | Key     | Action     |
//! |---------|------------|
//! | Arrows  | Pan Around |
//! | +/-     | Zoom In/Out|
//! | Enter   | Reset View |
//! | Space   | Pause/Play |
//! | R       | Reset Sim  |
//! | Q       | Quit       |
//!
//! Example usage:
//! ```rust
//! let window = MinimalRenderer::new(600, 600);
//! let mut sim = Sim::new();
//!
//! // Set up the simulation... //
//!
//! window.run(sim);
//! ```

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

const DEFAULT_COLOR: (u8, u8, u8, u8) = (40, 40, 40, 255);
const STROKE: f32 = 2.5;
const PAN_STEP: f64 = 20.0;
const ZOOM_STEP: f64 = 0.15;

pub struct MinimalRenderer {
    bg_color: (u8, u8, u8, u8),
    view_offset: Vec2,
    zoom: f64,
    event_loop: EventLoop<()>,
    time: Instant,
    context: GraphicsContext<Window>,
}

impl MinimalRenderer {
    /// Initialize the renderer with width & height.
    pub fn new(width: u16, height: u16) -> Self {
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

        MinimalRenderer {
            bg_color: DEFAULT_COLOR,
            view_offset: Vec2::new(0.0, 0.0),
            zoom: 1.0,
            event_loop,
            time: Instant::now(),
            context: unsafe { GraphicsContext::new(window) }.unwrap(),
        }
    }

    /// A builder method to give the window a non-default color upon creation (after calling new). Ex:
    ///
    /// ```rust
    /// let window = MinimalRenderer::new(WIDTH, HEIGHT).with_color(((R, G, B, A)));
    /// ```
    pub fn with_color(mut self: Self, color: (u8, u8, u8, u8)) -> Self {
        self.bg_color = color;
        self
    }

    /// Converts from an rgb color to the 32-bit binary format that softbuffer uses.
    ///
    /// Pixel format (u32): 00000000RRRRRRRRGGGGGGGGBBBBBBBB
    pub fn rgb_to_softbuffer(rgb: (u8, u8, u8)) -> u32 {
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
    fn dyn_width(context: &GraphicsContext<Window>) -> f64 {
        context.window().inner_size().width as f64
    }

    /// A method to dynamically get the window's width. see dyn_width.
    fn dyn_height(context: &GraphicsContext<Window>) -> f64 {
        context.window().inner_size().height as f64
    }

    /// Run the given simulation in a new window.
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
                    event:
                        DeviceEvent::Key(KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(code),
                            ..
                        }),
                    ..
                } => match code {
                    VirtualKeyCode::Left => {
                        self.view_offset.x -= PAN_STEP / std::f64::consts::E.powf(self.zoom - 1.0);
                    }
                    VirtualKeyCode::Right => {
                        self.view_offset.x += PAN_STEP / std::f64::consts::E.powf(self.zoom - 1.0);
                    }
                    VirtualKeyCode::Up => {
                        self.view_offset.y += PAN_STEP / std::f64::consts::E.powf(self.zoom - 1.0);
                    }
                    VirtualKeyCode::Down => {
                        self.view_offset.y -= PAN_STEP / std::f64::consts::E.powf(self.zoom - 1.0);
                    }
                    VirtualKeyCode::Equals => self.zoom += ZOOM_STEP,
                    VirtualKeyCode::Minus => self.zoom -= ZOOM_STEP,
                    VirtualKeyCode::Return => {
                        self.zoom = 1.0;
                        self.view_offset = Vec2::zero();
                    }
                    VirtualKeyCode::Space => sim.running = !sim.running,
                    VirtualKeyCode::Q => *control_flow = ControlFlow::Exit,
                    _ => (),
                },
                Event::MainEventsCleared => {
                    // update & render after other events are handled

                    let zoom = std::f64::consts::E.powf(self.zoom - 1.0);

                    // create the buffers
                    let mut draw_buffer = Pixmap::new(width as u32, height as u32).unwrap();
                    let mut framebuffer: Vec<u32> = Vec::new();

                    // create drawing styles
                    let mut style = Paint::default();
                    style.anti_alias = true;
                    let mut stroke = Stroke::default();
                    stroke.width = STROKE * (zoom as f32);

                    // paint the background
                    draw_buffer.fill(Color::from_rgba8(
                        self.bg_color.0,
                        self.bg_color.1,
                        self.bg_color.2,
                        self.bg_color.3,
                    ));

                    // draw the sim's particles
                    for particle in &sim.particles {
                        // transform from simulation space to window space,
                        // as the window is panned and zoomed.
                        let scale = ((zoom, 0.0), (0.0, zoom));
                        let identity = ((1.0, 0.0), (0.0, 1.0));
                        let pan = self.view_offset * -1.0;
                        let Vec2 { x, y } = particle
                            .pos
                            .affine_transformation(identity, pan)
                            .affine_transformation(scale, Vec2::zero());
                        let radius = particle.radius * zoom;
                        let (r, g, b, a) = particle.color;

                        let path = {
                            let mut pb = PathBuilder::new();
                            pb.push_circle(
                                (x + width / 2.0) as f32,
                                (height / 2.0 - y) as f32,
                                radius as f32,
                            );
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
