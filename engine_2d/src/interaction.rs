//! For interacting with the simulation while it's running.

use crate::physics::system::System;

pub fn pause_play(sim: &mut System) {
    sim.running = !sim.running;
}

pub fn step(sim: &mut System, dt: f64) {
    sim.step_forward(dt);
}

pub fn restart(_sim: &mut System) {
    todo!();
}
