use pixels_renderer::PixelsRenderer;
use rusty_particle_physics_2d::prelude::*;

const WIDTH: u32 = 650;
const HEIGHT: u32 = 650;

fn main() {
    let mut sim = Sim::new();
    PixelsRenderer::new(WIDTH, HEIGHT).run(&mut sim);
}
