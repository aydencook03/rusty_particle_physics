use rusty_particle_physics_2d::prelude::*;
use testing_renderer::PixelsRenderer;

fn main() {
    let mut sim = Sim::new();
    PixelsRenderer::new(500, 500).run(&mut sim);
}
