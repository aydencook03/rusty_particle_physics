use renderer_2d::Renderer;
use rusty_particle_physics_2d::prelude::*;

fn main() {
    let window = Renderer::new(600, 600);
    let mut sim = System::new();

    let gravity = Vec2::new(0.0, -400.0);

    let mut particle1 = Particle::new()
        .radius(10.0)
        .pos(0.0, 0.0)
        .vel(50.0, 70.0)
        .color(EARTH_BLUE);
    particle1.forces.push(gravity);
    sim.add_particle(particle1);

    let mut particle2 = Particle::new().radius(4.0).pos(90.0, 70.0).vel(20.0, -20.0);
    particle2.forces.push(gravity);
    sim.add_particle(particle2);

    /* IDEAL SYNTAX
    let gravity = Vec2::new(0.0, -400.0);

    let particle1 = sim.add_particle(
        Particle::new()
        .radius(10.0)
        .pos(0.0, 0.0)
        .vel(50.0, 70.0)
        .color(EARTH_BLUE)
    );
    let particle2 = sim.add_particle(
        Particle::new().radius(4.0).pos(90.0, 70.0).vel(20.0, -20.0)
    );
    particle1.forces.push(gravity);
    particle2.forces.push(gravity);

    sim.add_constraint(Constraint {
        particles: (particle1, particle2),
        function: |p1, p2| (p1.pos - )
    });
    */

    window.run(sim);
}
