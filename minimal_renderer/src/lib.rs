use rusty_particle_physics_2d::sim::Sim;

use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use softbuffer::GraphicsContext;

pub struct MinimalRenderer {
    pub width: u32,
    pub height: u32,
    pub bg_color: (u32, u32, u32),
    event_loop: EventLoop<()>,
    context: GraphicsContext<Window>,
    buffer: Vec<u32>,
}

impl MinimalRenderer {
    pub fn new(width: u32, height: u32) -> MinimalRenderer {
        // A "context" that provides a way to retrieve events from the system and the windows registered to it.
        // EventLoop::new() initializes everything that will be required to create windows.
        // For example on Linux creating an event loop opens a connection to the X or Wayland server.
        let event_loop = EventLoop::new();

        let window = {
            let size = PhysicalSize::new(width, height);
            WindowBuilder::new()
                .with_inner_size(size)
                .with_max_inner_size(size)
                .with_title("Simulation")
                .build(&event_loop)
                .unwrap()
        };

        let buffer_len: usize = (width * height).try_into().unwrap();

        let bg_color: (u32, u32, u32) = (70, 70, 70);

        let start_color = MinimalRenderer::rgb_to_u32(bg_color);

        MinimalRenderer {
            width,
            height,
            bg_color,
            event_loop,
            context: unsafe { GraphicsContext::new(window) }.unwrap(),
            buffer: vec![start_color; buffer_len],
        }
    }

    /// Converts from an rgb color to the format that softbuffer uses.
    /// Pixel format (u32): 00000000RRRRRRRRGGGGGGGGBBBBBBBB
    pub fn rgb_to_u32(rgb: (u32, u32, u32)) -> u32 {
        let (r, g, b) = rgb;
        let r = r << 16;
        let g = g << 8;
        let b = b << 0;

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
                    self.context.set_buffer(
                        &self.buffer,
                        self.width.try_into().unwrap(),
                        self.height.try_into().unwrap(),
                    );
                }
                _ => (),
            }
        });
    }
}
