pub enum RenderMode {
    RealTime { fps: u8 },
    BakedAnimation { fps: u8, length: f64 },
}

pub trait Renderer {
    type Name;
    fn init() -> Self::Name;
    fn run(self: &Self);
    // model after the web's requestAnimationFrame. calls run. provide default implementation.
    fn request_next_frame(self: &Self, delay: f64);
}
