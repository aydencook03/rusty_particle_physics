use rusty_particle_physics_2d::sim::Sim;

use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use softbuffer::GraphicsContext;

pub struct MinimalRenderer {
    pub width: u16,
    pub height: u16,
    bg_color_u32: u32,
    buffer_len: usize,
    event_loop: EventLoop<()>,
    context: GraphicsContext<Window>,
    buffer: Vec<u32>,
}

impl MinimalRenderer {
    pub fn new(width: u16, height: u16, color: (u8, u8, u8)) -> MinimalRenderer {
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

        let width: u16 = window.inner_size().width.try_into().unwrap();
        let height: u16 = window.inner_size().height.try_into().unwrap();

        let buffer_len: usize = (width * height).try_into().unwrap();

        let bg_color_u32 = MinimalRenderer::rgb_to_u32(color);

        MinimalRenderer {
            width,
            height,
            bg_color_u32,
            buffer_len,
            event_loop,
            context: unsafe { GraphicsContext::new(window) }.unwrap(),
            buffer: vec![bg_color_u32; buffer_len],
        }
    }

    /// Converts from an rgb color to the 32-bit format that softbuffer uses.
    /// Pixel format (u32): 00000000RRRRRRRRGGGGGGGGBBBBBBBB
    pub fn rgb_to_u32(rgb: (u8, u8, u8)) -> u32 {
        let (r, g, b) = rgb;
        let r = (r as u32) << 16;
        let g = (g as u32) << 8;
        let b = (b as u32) << 0;

        r | g | b
    }

    pub fn run(mut self: Self, _sim: &mut Sim) {
        let event_loop = self.event_loop;
        event_loop.run(move |event, _, control_flow| {
            // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
            // dispatched any events. This is ideal for games and similar applications.
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    println!("The close button was pressed. Stopping...");
                    *control_flow = ControlFlow::Exit
                }
                Event::MainEventsCleared => {
                    // update & render

                    // clear the screen
                    self.buffer = vec![self.bg_color_u32; self.buffer_len];

                    // write the contents of self.buffer to the window buffer
                    self.context
                        .set_buffer(&self.buffer, self.width, self.height);
                }
                _ => (),
            }
        });
    }
}
