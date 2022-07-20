use rusty_particle_physics_2d::sim::Sim;

use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use std::time::Instant;

use softbuffer::GraphicsContext;

use tiny_skia::{Color, FillRule, Paint, PathBuilder, Pixmap, Stroke, Transform};

const DEFAULT_COLOR: (u8, u8, u8) = (70, 70, 70);

pub struct MinimalRenderer {
    pub width: u16,
    pub height: u16,
    bg_color: (u8, u8, u8),
    event_loop: EventLoop<()>,
    time: Instant,
    context: GraphicsContext<Window>,
    framebuffer: Vec<u32>,
    draw_buffer: Pixmap,
    style: Paint<'static>,
    black_style: Paint<'static>,
    stroke_style: Stroke,
}

impl MinimalRenderer {
    pub fn new(width: u16, height: u16) -> Self {
        // A "context" that provides a way to retrieve events from the system and the windows registered to it.
        // EventLoop::new() initializes everything that will be required to create windows.
        // For example on Linux creating an event loop opens a connection to the X or Wayland server.
        let event_loop = EventLoop::new();

        let window = {
            let size = LogicalSize::new(width, height);
            WindowBuilder::new()
                .with_inner_size(size)
                .with_max_inner_size(size)
                .with_title("Simulation")
                .build(&event_loop)
                .unwrap()
        };

        let width: u16 = window.inner_size().width.try_into().unwrap();
        let height: u16 = window.inner_size().height.try_into().unwrap();

        let mut style = Paint::default();
        style.anti_alias = true;

        let mut black_style = Paint::default();
        black_style.set_color_rgba8(0, 0, 0, 255);
        black_style.anti_alias = true;

        let mut stroke_style = Stroke::default();
        stroke_style.width = 2.0;

        MinimalRenderer {
            width,
            height,
            bg_color: DEFAULT_COLOR,
            event_loop,
            time: Instant::now(),
            context: unsafe { GraphicsContext::new(window) }.unwrap(),
            framebuffer: vec![0; (width as usize) * (height as usize)],
            draw_buffer: Pixmap::new(width as u32, height as u32).unwrap(),
            style,
            black_style,
            stroke_style,
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

    pub fn with_color(mut self: Self, color: (u8, u8, u8)) -> Self {
        self.bg_color = color;
        self
    }

    pub fn width(self: &Self) -> f64 {
        self.width as f64
    }

    pub fn height(self: &Self) -> f64 {
        self.height as f64
    }

    pub fn run(mut self: Self, mut sim: Sim<'static>) {
        self.event_loop.run(move |event, _, control_flow| {
            // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
            // dispatched any events. This is ideal for games and similar applications.
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    // stop the event loop, therefore closing the window
                    *control_flow = ControlFlow::Exit
                }
                Event::MainEventsCleared => {
                    // update & render after other events are handled

                    // clear the buffers
                    self.draw_buffer.fill(Color::from_rgba8(
                        self.bg_color.0,
                        self.bg_color.1,
                        self.bg_color.2,
                        255,
                    ));
                    self.framebuffer.clear();

                    // draw the sim's particles
                    for particle in &sim.particles {
                        let x = particle.pos.x as f32;
                        let y = particle.pos.y as f32;
                        let radius = particle.radius as f32;
                        let (r, g, b, a) = particle.color;
                        self.style.set_color_rgba8(r, g, b, a);

                        let path = {
                            let mut pb = PathBuilder::new();
                            pb.push_circle(x, y, radius);
                            pb.finish().unwrap()
                        };

                        self.draw_buffer.stroke_path(
                            &path,
                            &self.black_style,
                            &self.stroke_style,
                            Transform::identity(),
                            None,
                        );
                        self.draw_buffer.fill_path(
                            &path,
                            &self.style,
                            FillRule::Winding,
                            Transform::identity(),
                            None,
                        );
                    }

                    // copy the contents from draw_buffer to framebuffer w/ required format
                    for color in self.draw_buffer.pixels() {
                        let rgb = (color.red(), color.green(), color.blue());
                        self.framebuffer
                            .push(MinimalRenderer::rgb_to_softbuffer(rgb));
                    }

                    // write the contents of self.buffer to the window buffer
                    self.context
                        .set_buffer(&self.framebuffer, self.width, self.height);

                    let elapsed = (Instant::now().duration_since(self.time).as_micros() as f64)
                        * (10.0_f64).powi(-6);
                    sim.step_simulation(elapsed);
                    self.context
                        .window_mut()
                        .set_title(format!("Simulation: {:.0} fps", 1.0 / elapsed).as_str());
                    self.time = Instant::now();
                }
                _ => (),
            };
        });
    }
}
