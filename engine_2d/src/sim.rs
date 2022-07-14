use crate::matter::particle::Particle;
use crate::dynamics::force::Force;
use crate::dynamics::constraint::Constraint;

#[derive(Default)]
pub struct Sim {
    pub updates_per_sec: u8,
    dt: f64,
    pub renders_per_sec: u8,
    pub constraint_passes: u8,
    pub particles: Vec<Particle>,
    pub constraints: Vec<Constraint>,
    pub forces: Vec<Force>,
}

impl Sim {
    /// Construct a Sim
    pub fn new() -> Sim {
        Sim {
            updates_per_sec: 60,
            renders_per_sec: 60,
            constraint_passes: 3,
            dt: 1.0 / 60.0,
            ..Default::default()
        }
    }

    fn handle_constraints(self: &Self) {
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

    pub fn step_simulation(self: &mut Self) {
        // what should the order of these be?
        // What should I do about dt?
        self.handle_constraints();
        self.send_forces_to_particles();
        self.update_particles(self.dt);
    }

    pub fn render(self: &Self) {
        todo!();
    }

    pub fn run(self: &Self) {
        todo!();
    }
}
