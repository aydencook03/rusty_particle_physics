mod force;
mod constraint;

use crate::particle::Particle;
use force::Force;
use constraint::Constraint;

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

    fn update_particles(self: &Self) {
        for particle in self.particles {
            particle.symplectic_euler_update();
        }
    }

    fn add_particle(self: &mut Self) {
        todo!();
    }

    /* fn add_body(self: &mut Self, body: Body) {
        body.push_to_vec(self.particles);
    } */

    fn run(self: &Self) {
        /*
        self.send_forces_to_particles();
        self.update_particles(self.dt);
        self.handle_static_constraints();
        */
    }
}