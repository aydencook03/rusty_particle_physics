//! For interacting with the simulation while it's running.

use crate::physics::sim::Sim;

pub fn pause_play(sim: &mut Sim) {
    sim.running = !sim.running;
}

pub fn step(sim: &mut Sim, dt: f64) {
    sim.step_simulation(dt);
}

pub fn restart(_sim: &mut Sim) {
    todo!();
}
