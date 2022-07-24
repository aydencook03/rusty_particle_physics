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

use crate::sim_dynamics::force::Force;
use crate::particle::Particle;

#[derive(Default)]
pub struct Sim<'a> {
    pub running: bool,
    pub time: f64,
    pub particles: Vec<Particle>,
    pub forces: Vec<Force<'a>>,
}

impl<'a> Sim<'a> {
    /// Construct a default Sim
    pub fn new() -> Sim<'a> {
        Sim {
            running: true,
            ..Default::default()
        }
    }

    pub fn step_simulation(self: &mut Self, dt: f64) {
        if self.running {
            self.calculate_forces();
            self.update_particles(dt);
            self.time += dt;
        }
    }
}

impl<'a> Sim<'a> {
    pub fn add_particle(self: &mut Self, particle: Particle) {
        self.particles.push(particle);
    }

    pub fn remove_particle(self: &mut Self, index: usize) {
        self.particles.swap_remove(index);
    }

    pub fn clear_particles(self: &mut Self) {
        self.particles.clear();
    }

    pub fn add_force(self: &mut Self, force: Force<'a>) {
        self.forces.push(force);
    }

    pub fn remove_force(self: &mut Self, index: usize) {
        self.forces.swap_remove(index);
    }

    pub fn clear_forces(self: &mut Self) {
        self.forces.clear();
    }
}

impl<'a> Sim<'a> {
    fn calculate_forces(self: &mut Self) {
        for force in &mut self.forces {
            force.send();
        }
    }

    fn update_particles(self: &mut Self, dt: f64) {
        for particle in &mut self.particles {
            particle.update(dt);
        }
    }
}
