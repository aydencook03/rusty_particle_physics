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
//! Example usage:
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

use crate::dynamics::constraint::Constraint;
use crate::dynamics::force::Force;
use crate::particle::Particle;

#[derive(Default)]
pub struct Sim<'a> {
    pub running: bool,
    pub time: f64,
    pub constraint_passes: u8,
    pub particles: Vec<Particle>,
    pub constraints: Vec<Constraint<'a>>,
    pub forces: Vec<Force<'a>>,
}

impl<'a> Sim<'a> {
    /// Construct a default Sim
    pub fn new() -> Sim<'a> {
        Sim {
            running: true,
            constraint_passes: 3,
            ..Default::default()
        }
    }

    pub fn step_simulation(self: &mut Self, dt: f64) {
        if self.running {
            // what should the order of these be?
            self.handle_constraints();
            self.send_forces_to_particles();
            self.update_particles(dt);
            self.time += dt;
        }
    }

    fn handle_constraints(self: &mut Self) {
        for _ in 0..self.constraint_passes {
            for constraint in &mut self.constraints {
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
            particle.update(dt);
        }
    }
}
