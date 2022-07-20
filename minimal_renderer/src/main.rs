use minimal_renderer::MinimalRenderer;
use rusty_particle_physics_2d::prelude::*;

fn main() {
    let window = MinimalRenderer::new(600, 600);
    let mut sim = Sim::new();

    sim.particles.push({
        let mut particle = Particle::new();
        particle.radius = 10.0;
        particle.pos = Vec2::new(50.0, 100.0);
        particle.vel = Vec2::new(50.0, -70.0);
        particle.forces.push(Vec2::new(0.0, 400.0));
        particle
    });

    window.run(sim);
}
