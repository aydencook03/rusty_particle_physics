use crate::dynamics::constraint::Constraint;
use crate::dynamics::force::Force;
use crate::matter::particle::Particle;
//use crate::rendering::Renderer;

#[derive(Default)]
pub struct Sim {
    pub updates_per_sec: u8,
    //pub renderer: Option<impl Renderer>,
    pub constraint_passes: u8,
    pub particles: Vec<Particle>,
    pub constraints: Vec<Constraint>,
    pub forces: Vec<Force>,
}

impl Sim {
    /// Construct a Sim
    pub fn new(updates_per_sec: u8 /*renderer: Option<impl Renderer>*/) -> Sim {
        Sim {
            updates_per_sec,
            //renderer,
            constraint_passes: 3,
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
            particle.update(dt);
        }
    }

    pub fn step_simulation(self: &mut Self) {
        // what should the order of these be?
        // What should I do about dt?
        self.handle_constraints();
        self.send_forces_to_particles();
        self.update_particles(1.0 / (self.updates_per_sec as f64));
    }

    pub fn render(self: &Self) {
        todo!();
    }

    pub fn run(self: &Self) {
        todo!();
    }
}
