/// A 2d euclidean vector
#[derive(Copy, Clone, Default)]
pub struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    /// Default constructor function: returns a zero Vec2 (using Default for f64)
    fn new() -> Vec2 {
        Vec2 {
            ..Default::default()
        }
    }

    fn new_euclid(x: f64, y: f64) -> Vec2 {
        todo!();
    }

    fn new_polar(r: f64, angle: f64) -> Vec2 {
        todo!();
    }

    /// Returns the magnitude of the Vec2
    fn mag(self: &Self) -> f64 {
        (self.x * self.x + self.y * self.y).powf(0.5_f64)
    }
}
