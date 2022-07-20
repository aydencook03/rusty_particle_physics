use minimal_renderer::MinimalRenderer;
use rusty_particle_physics_2d::prelude::*;

const WIDTH: u16 = 300;
const HEIGHT: u16 = 300;

fn main() {
    let window = MinimalRenderer::new(WIDTH, HEIGHT);
    let mut sim = Sim::new();

    sim.particles.push({
        let mut particle = Particle::new();
        particle.radius = 5.0;
        particle.pos = Vec2::new(50.0, 100.0);
        particle.vel = Vec2::new(50.0, -70.0);
        particle.forces.push(Vec2::new(0.0, 400.0));
        particle
    });

    window.run(sim);
}
