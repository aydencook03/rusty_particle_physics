use minimal_renderer::MinimalRenderer;
use rusty_particle_physics_2d::prelude::*;

const WIDTH: u16 = 600;
const HEIGHT: u16 = 600;

fn main() {
    let window = MinimalRenderer::new(WIDTH, HEIGHT);
    let mut sim = Sim::new();

    window.run(&mut sim);
}
