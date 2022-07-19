use rusty_particle_physics_2d::sim::Sim;

use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use softbuffer::GraphicsContext;

pub struct MinimalRenderer {
    pub width: u32,
    pub height: u32,
    event_loop: EventLoop<()>,
    pub context: GraphicsContext<Window>,
    pub buffer: Vec<u32>,
}

impl MinimalRenderer {
    pub fn new(width: u32, height: u32) -> MinimalRenderer {
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

        let buffer_len = width * height;

        MinimalRenderer {
            width,
            height,
            event_loop,
            context: unsafe { GraphicsContext::new(window) }.unwrap(),
            buffer: Vec::with_capacity(buffer_len.try_into().unwrap()),
        }
    }

    pub fn run(self: Self, _sim: &mut Sim) {
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
                }
                _ => (),
            }
        });
    }
}
