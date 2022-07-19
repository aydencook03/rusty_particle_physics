use rusty_particle_physics_2d::sim::Sim;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
    dpi::LogicalSize,
};

use pixels::{Pixels, SurfaceTexture};

pub struct PixelsRenderer {
    event_loop: EventLoop<()>,
    window: Window,
    pixels: Pixels,
}

impl PixelsRenderer {
    pub fn new(width: u32, height: u32) -> PixelsRenderer {
        let event_loop = EventLoop::new();
        
        let window = {
            let size = LogicalSize::new(width, height);
            WindowBuilder::new()
            .with_inner_size(size)
            .build(&event_loop)
            .unwrap()
        };

        let pixels = {
            let size = window.inner_size();
            let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
            Pixels::new(width, height, surface_texture).unwrap()
        };

        PixelsRenderer {
            window,
            event_loop,
            pixels,
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
                    println!("The close button was pressed; stopping");
                    *control_flow = ControlFlow::Exit
                },
                Event::MainEventsCleared => {
                    // update & render
                },
                _ => ()
            }
        });
    }
}