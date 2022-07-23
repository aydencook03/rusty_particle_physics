use crate::sim::Sim;

pub fn pause_play(sim: &mut Sim) {
    sim.running = !sim.running;
}

pub fn restart(_sim: &mut Sim) {
    todo!();
}
