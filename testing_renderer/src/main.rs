use rusty_particle_physics_2d::rendering::IsRenderer;
use rusty_particle_physics_2d::sim::Sim;

struct BevyRenderer;

impl IsRenderer for BevyRenderer {
    fn init(self: &Self, _width: u32, _height: u32) {}
    fn paint(self: &Self) {}
    fn events(self: &Self) {}
    fn set_real_time(self: &Self) {}
    fn set_baked(self: &Self) {}
    fn delay(self: &Self, _dt: f64) {}
}

fn main() {
    let mut sim = Sim::new().set_renderer(Box::new(BevyRenderer), 500, 500);
    sim.run(60);
}
