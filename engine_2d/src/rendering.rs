pub trait IsRenderer {
    type Output;
    /// Initialize window, surface, events, etc.
    fn new(width: u32, height: u32) -> Self::Output;
    fn paint(self: &Self);
    fn events(self: &Self);
    fn set_real_time(self: &Self);
    fn set_baked(self: &Self);
    // model after the web's requestAnimationFrame. calls paint. provide default implementation.
    /// Responsible for causing delays to sync simulation time and real time.
    /// Is only used when the simulation is needed to run in real time, not when creating an animation.
    fn delay(self: &Self, dt: f64);
    /// Start the event loop, step the simulation, manage time delays, etc.
    fn run(self: &Self);
}

/*
impl Default for Box<dyn IsRenderer> {
    fn default() -> Self {
        Box::new(SimpleDelayRenderer)
    }
}
*/

/// The default renderer that only has the functionality of delaying the simulation
pub struct SimpleDelayRenderer;

impl IsRenderer for SimpleDelayRenderer {
    type Output = SimpleDelayRenderer;
    fn new(_width: u32, _height: u32) -> Self::Output {
        SimpleDelayRenderer
    }
    fn paint(self: &Self) {}
    fn events(self: &Self) {}
    fn set_real_time(self: &Self) {}
    fn set_baked(self: &Self) {}
    fn delay(self: &Self, _dt: f64) {}
    fn run(self: &Self) {}
}
