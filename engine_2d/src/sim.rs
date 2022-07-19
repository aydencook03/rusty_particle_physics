use crate::dynamics::constraint::Constraint;
use crate::dynamics::force::Force;
use crate::matter::particle::Particle;
//use crate::rendering::IsRenderer;

#[derive(Default)]
pub struct Sim<'a> {
    pub running: bool,
    pub time: f64,
    //pub dt: f64,
    //pub renderer: Box<dyn IsRenderer>,
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
        //.set_fps(60)
        //.set_renderer(Default::default(), 0, 0)
    }

    pub fn dt_from_fps(fps: u8) -> f64 {
        1.0 / (fps as f64)
    }

    /*
    pub fn set_fps(mut self: Self, simulation_fps: u8) -> Sim<'a> {
        self.dt = Sim::dt_from_fps(simulation_fps);
        self
    }

    pub fn set_renderer(
        mut self: Self,
        renderer: Box<dyn IsRenderer>,
        width: u32,
        height: u32,
    ) -> Sim<'a> {
        self.renderer = renderer;
        self.renderer.init(width, height);
        self
    }
    */

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

    pub fn step_simulation(self: &mut Self, dt: f64) {
        if self.running {
            // what should the order of these be?
            self.handle_constraints();
            self.send_forces_to_particles();
            self.update_particles(dt);
            self.time += dt;
        }
    }

    /*
    pub fn run(self: &mut Self, _render_fps: u8) {
        // This doesn't yet allow simulation_fps and render_fps to be different
        while self.running {
            // what should the order of these be?
            self.renderer.paint();
            self.step_simulation(self.dt);
            self.renderer.delay(self.dt);
            self.renderer.events();
        }
    }

    pub fn create_animation(self: &mut Self, _animation_fps: u8, _animation_length: f64) {
        todo!();
    }
    */
}
