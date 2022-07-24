use renderer_2d::Renderer;
use rusty_particle_physics_2d::prelude::*;

fn main() {
    let window = Renderer::new(600, 600);
    let mut sim = Sim::new();

    sim.add_particle({
        let mut particle = Particle::new()
            .radius(10.0)
            .pos(0.0, 0.0)
            .vel(50.0, 70.0)
            .color(EARTH_BLUE);
        particle.forces.push(Vec2::new(0.0, -400.0));
        particle
    });
    sim.add_particle({
        let mut particle = Particle::new().radius(4.0).pos(90.0, 70.0).vel(20.0, -20.0);
        particle.forces.push(Vec2::new(0.0, -400.0));
        particle
    });

    window.run(sim);
}
