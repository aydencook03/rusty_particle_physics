//! Provides the `Sim`, which is the thing that controls everything.
//!
//! Technically, one could use the other modules (such as `vec2`, `particle`, etc) all by themselves by manually creating
//! everything and making loops to call everything's update functions, etc.
//!
//! However, this would be very tedious and not too helpful. Instead, the main interface to this engine is provided by this
//! `Sim` struct. This struct manages lists of all of the simulation's active particles, forces, constraints, etc, and it
//! automatically manages their updating, timekeeping, etc.
//!
//! Once setting up the simulation with all of the particles, constraint, and forces, one simply needs to call the
//! Sim::step_simulation method with the desired (or calculated) timestep, and it will handle all of the physics.
//!
//! # Example usage:
//!
//! ```rust
//! let mut sim = Sim::new();
//!
//! // Add particles, forces, constraints, etc //
//!
//! loop {
//!     sim.step_simulation(1.0/FPS);
//! }
//! ```

use crate::physics::constraint::Constraint;
use crate::physics::force::Force;
use crate::physics::particle::Particle;

#[derive(Default)]
pub struct Sim<'a> {
    pub running: bool,
    pub substeps: u32,
    time: f64,
    particles: Vec<Particle>,
    forces: Vec<Force<'a>>,
    constraints: Vec<Constraint>,
}

impl<'a> Sim<'a> {
    /// Construct a default Sim
    pub fn new() -> Sim<'a> {
        Sim {
            running: true,
            substeps: 1,
            ..Default::default()
        }
    }

    pub fn step_simulation(self: &mut Self, dt: f64) {
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
                    Constraint::solver(constraint, sub_dt);
                }
                for particle in &mut self.particles {
                    particle.update_vel(sub_dt);
                }
            }
            // TODO: check for collisions, clear particle forces, remove broken constraints... //
            self.time += dt;
        }
    }

    /// Get the current simulation time
    pub fn time(self: &Self) -> f64 {
        self.time
    }

    /// Get a reference to the simulation's particles
    pub fn particles(self: &mut Self) -> &[Particle] {
        &self.particles
    }

    /// Add a new particle to the simulation
    pub fn add_particle(self: &mut Self, particle: Particle) {
        self.particles.push(particle);
    }
}
