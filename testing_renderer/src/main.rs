use rusty_particle_physics_2d::sim::*;

fn main() {
    let _simulation = Sim::new().set_baked(60, 60, 20.0);
    _simulation.set_real_time(60, 60);

    println!("Hello");
}
