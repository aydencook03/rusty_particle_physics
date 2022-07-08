use crate::matter::particle::Particle;
use crate::matter::body::Body;
use crate::simulation::force::Force;
use crate::simulation::constraint::Constraint;

pub struct Sim {
    updates_per_sec: u8,
    constraint_passes: u8,
    particles: Vec<Particle>,
    static_constraints: Vec<Constraint>,
    forces: Vec<Force>,
}

impl Sim {
    fn new() -> Sim {
        Sim {
            updates_per_sec: 60,
            constraint_passes: 3,
            ..Default::default()
        }
    }

    fn handle_static_constraints(self: &Self) {
        for _ in 0..self.constraint_passes {
            for constraint in self.static_constraints {
                constraint.handle_as_static_constraint();
            }
        }
    }

    fn send_forces_to_particles(self: &mut Self) {
        for force in self.forces {
            force.send_force();
        }
    }

    fn update_particles(self: &Self, dt: f64) {
        for particle in self.particles {
            particle.symplectic_euler_update(dt);
        }
    }

    fn add_particle(self: &mut Self) {
        todo!();
    }

    fn add_body(self: &mut Self, body: Body) {
        todo!();
    }

    fn run(self: &Self) {
        self.send_forces_to_particles();
        self.update_particles(1.0 / self.updates_per_sec);
        self.handle_static_constraints();
    }
}
