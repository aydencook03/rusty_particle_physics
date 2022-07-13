use crate::matter::particle::Particle;
use crate::matter::body::Body;
use crate::dynamics::force::Force;
use crate::dynamics::constraint::Constraint;

#[derive(Default)]
pub struct Sim {
    updates_per_sec: u8,
    renders_per_sec: u8,
    constraint_passes: u8,
    particles: Vec<Particle>,
    constraints: Vec<Constraint>,
    forces: Vec<Force>,
}

impl Sim {
    /// Constructs a default Sim
    fn new() -> Sim {
        Sim {
            updates_per_sec: 60,
            renders_per_sec: 60,
            constraint_passes: 3,
            ..Default::default()
        }
    }

    fn handle_static_constraints(self: &Self) {
        for _ in 0..self.constraint_passes {
            for constraint in &self.constraints {
                constraint.handle();
            }
        }
    }

    fn send_forces_to_particles(self: &mut Self) {
        for force in &mut self.forces {
            force.send_force();
        }
    }

    fn update_particles(self: &mut Self, dt: f64) {
        for particle in &mut self.particles {
            particle.symplectic_euler_update(dt);
        }
    }

    fn add_particle(self: &mut Self) {
        todo!();
    }

    fn add_body(self: &mut Self, body: Body) {
        todo!();
    }

    fn step_simulation(self: &Self) {
        todo!();
    }

    fn render(self: &Self) {
        todo!();
    }

    fn run(self: &Self) {
        todo!();
    }
}
