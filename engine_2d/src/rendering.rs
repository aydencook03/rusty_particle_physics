pub enum RenderMode {
    RealTime { fps: u8 },
    BakedAnimation { fps: u8, length: f64 },
}

pub trait Renderer {
    const MODE: RenderMode;
}