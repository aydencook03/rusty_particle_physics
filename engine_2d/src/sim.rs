use crate::dynamics::constraint::Constraint;
use crate::dynamics::force::Force;
use crate::matter::particle::Particle;
use crate::rendering::Renderer;

#[derive(Default)]
pub enum RunMode {
    #[default]
    None,
    RealTime {
        simulation_fps: u8,
        render_fps: u8,
    },
    Baked {
        simulation_fps: u8,
        animation_fps: u8,
        animation_length: f64,
    },
}

#[derive(Default)]
pub struct Sim<'a> {
    pub run_mode: RunMode,
    pub renderer: Option<Box<dyn Renderer>>,
    pub constraint_passes: u8,
    pub particles: Vec<Particle>,
    pub constraints: Vec<Constraint<'a>>,
    pub forces: Vec<Force<'a>>,
    dt: f64,
}

impl<'a> Sim<'a> {
    /// Construct a Sim
    pub fn new(renderer: Option<Box<dyn Renderer>>) -> Sim<'a> {
        Sim {
            renderer,
            constraint_passes: 3,
            ..Default::default()
        }
        .set_real_time(60, 60)
    }

    pub fn set_real_time(mut self: Self, simulation_fps: u8, render_fps: u8) -> Sim<'a> {
        self.run_mode = RunMode::RealTime {
            simulation_fps,
            render_fps,
        };
        self.dt = 1.0 / (simulation_fps as f64);

        self
    }

    pub fn set_baked(
        mut self: Self,
        simulation_fps: u8,
        animation_fps: u8,
        animation_length: f64,
    ) -> Sim<'a> {
        self.run_mode = RunMode::Baked {
            simulation_fps,
            animation_fps,
            animation_length,
        };
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
