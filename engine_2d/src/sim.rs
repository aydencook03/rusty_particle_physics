use crate::dynamics::constraint::Constraint;
use crate::dynamics::force::Force;
use crate::matter::particle::Particle;
use crate::rendering::Renderer;

pub enum RunMode {
    RealTime { simulation_fps: u8, render_fps: u8 },
    Baked { simulation_fps: u8, animation_fps: u8, animation_length: f64 },
}

impl Default for RunMode {
    fn default() -> Self {
        RunMode::RealTime { simulation_fps: 60, render_fps: 60 }
    }
}

#[derive(Default)]
pub struct Sim {
    pub run_mode: RunMode,
    pub renderer: Option<Box<dyn Renderer>>,
    pub constraint_passes: u8,
    pub particles: Vec<Particle>,
    pub constraints: Vec<Constraint>,
    pub forces: Vec<Force>,
    dt: f64,
}

impl Sim {
    /// Construct a Sim
    pub fn new(renderer: Option<Box<dyn Renderer>>) -> Sim {
        Sim {
            renderer,
            constraint_passes: 3,
            ..Default::default()
        }.as_real_time(60, 60)
    }

    pub fn as_real_time(mut self: Self, simulation_fps: u8, render_fps: u8) -> Sim {
        self.run_mode = RunMode::RealTime { simulation_fps, render_fps };
        self.dt = 1.0 / (simulation_fps as f64);

        self
    }

    pub fn as_baked(mut self: Self, simulation_fps: u8, animation_fps: u8, animation_length: f64) -> Sim {
        self.run_mode = RunMode::Baked { simulation_fps, animation_fps, animation_length };
        self.dt = 1.0 / (simulation_fps as f64);

        self
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
        self.update_particles(self.dt);
    }

    pub fn render(self: &Self) {
        todo!();
    }

    pub fn run(self: &Self) {
        todo!();
    }
}
