use crate::dynamics::constraint::Constraint;
use crate::dynamics::force::Force;
use crate::matter::particle::Particle;

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

    pub fn dt_from_fps(fps: u32) -> f64 {
        1.0 / (fps as f64)
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
