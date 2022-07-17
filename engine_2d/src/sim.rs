use crate::dynamics::constraint::Constraint;
use crate::dynamics::force::Force;
use crate::matter::particle::Particle;
use crate::rendering::IsRenderer;

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
    pub renderer: Box<dyn IsRenderer>,
    pub running: bool,
    pub time: f64,
    dt: f64,
    pub constraint_passes: u8,
    pub particles: Vec<Particle>,
    pub constraints: Vec<Constraint<'a>>,
    pub forces: Vec<Force<'a>>,
}

impl<'a> Sim<'a> {
    /// Construct a Sim
    pub fn new() -> Sim<'a> {
        Sim {
            constraint_passes: 3,
            ..Default::default()
        }
        .set_real_time(60, 60)
    }

    pub fn set_renderer(mut self: Self, renderer: Option<Box<dyn IsRenderer>>) -> Sim<'a> {
        self.renderer = match renderer {
            None => Default::default(),
            Some(the_renderer) => the_renderer,
        };
        self
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
        self.time += self.dt;
    }

    pub fn run(self: &mut Self) {
        while self.running {
            // what should the order of these be?
            self.renderer.paint();
            self.step_simulation();
            self.renderer.delay();
            self.renderer.events();
        }
    }
}
