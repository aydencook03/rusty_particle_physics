use crate::sim::Sim;

pub trait Renderer {
    fn init(self: &Self, sim: &Sim);
    fn run(self: &Self);
    fn paint(self: &Self, data_source: &Sim);
    // model after the web's requestAnimationFrame. calls run. provide default implementation.
    fn request_next_frame(self: &Self, delay: f64);
}
