//! Provides the `System`, which is a collection of interacting particles, global forces, and constraints.
//!
//! Once setting up the simulation with all of the particles, constraint, and forces, one simply needs to
//! call the System::step_forward method with the desired (or calculated) timestep, and it will
//! handle all of the physics.
//!
//! # Example usage:
//!
//! ```rust
//! let mut sim = System::new();
//!
//! // Add particles, forces, constraints, etc //
//!
//! loop {
//!     sim.step_forward(1.0/FPS);
//! }
//! ```

use crate::physics::constraint::Constraint;
use crate::physics::force::Force;
use crate::physics::particle::Particle;

/// A system is a collection of interacting particles, global forces, and constraints.
#[derive(Default)]
pub struct System {
    pub running: bool,
    pub substeps: u32,
    time: f64,
    pub particles: Vec<Particle>,
    particle_id_counter: u32,
    pub forces: Vec<Force>,
    pub constraints: Vec<Constraint>,
}

impl System {
    /// Construct a a new system
    pub fn new() -> System {
        System {
            running: true,
            substeps: 1,
            ..Default::default()
        }
    }

    /// Step the system's simulation forward in time by amount dt.
    pub fn step_forward(self: &mut Self, dt: f64) {
        if self.running {
            let sub_dt = dt / self.substeps as f64;
            for force in &self.forces {
                force.send();
            }
            for _ in 0..self.substeps {
                for particle in &mut self.particles {
                    particle.update(sub_dt);
                }
                for constraint in &self.constraints {
                    constraint.project(sub_dt);
                }
                for particle in &mut self.particles {
                    particle.update_vel(sub_dt);
                }
            }
            // TODO: check and handle collisions, clear particle forces, remove broken constraints... //
            self.time += dt;
        }
    }

    /// Get the current simulation time
    pub fn time(self: &Self) -> f64 {
        self.time
    }

    /// Add a new particle to the system. Returns a reference to that particle.
    pub fn add_particle(self: &mut Self, particle: Particle) {
        self.particles.push(particle.id(self.particle_id_counter));
        self.particle_id_counter += 1;
    }

    pub fn add_constraint(self: &mut Self, constraint: Constraint) {
        self.constraints.push(constraint);
    }
}
