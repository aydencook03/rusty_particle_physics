/// A 2d euclidean vector
#[derive(Copy, Clone, Default)]
pub struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    /// Constructor function for default Vec2: returns a zero Vec2 (using Default for f64)
    pub fn new() -> Vec2 {
        Vec2 {
            ..Default::default()
        }
    }

    /// Create a Vec2 using x and y components
    pub fn new_rect(x: f64, y: f64) -> Vec2 {
        Vec2 {
            x,
            y,
        }
    }

    /// Create a Vec2 using polar components
    pub fn new_polar(r: f64, angle: f64) -> Vec2 {
        Vec2 {
            x: r * angle.cos(),
            y: r * angle.sin(),
        }
    }

    /// Returns the magnitude of the Vec2
    pub fn mag(self: &Self) -> f64 {
        (self.x * self.x + self.y * self.y).powf(0.5_f64)
    }
}
