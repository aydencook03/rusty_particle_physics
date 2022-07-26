use renderer_2d::Renderer;
use rusty_particle_physics_2d::prelude::*;

fn main() {
    let window = Renderer::new(600, 600);
    let mut sim = Sim::new();

    let gravity = Vec2::new(0.0, -400.0);

    let mut particle1 = Particle::new()
        .radius(10.0)
        .pos(0.0, 0.0)
        .vel(50.0, 70.0)
        .color(EARTH_BLUE);
    particle1.forces.push(gravity);

    let mut particle2 = Particle::new().radius(4.0).pos(90.0, 70.0).vel(20.0, -20.0);
    particle2.forces.push(gravity);

    // CONSTRAINT TESTING
    // lock particle distance to 20.0 from center
    particle1.constraints.push(Constraint {
        function: Box::new(|pos| (pos - Vec2::new(0.0, 0.0)).mag() - 20.0),
        stiffness: 1.0,
        kind: ConstraintKind::Equality,
        broken: false,
    });

    sim.add_particle(particle1);
    sim.add_particle(particle2);

    window.run(sim);
}
