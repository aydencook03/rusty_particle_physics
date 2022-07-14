/// A 2d euclidean vector
#[derive(Copy, Clone, Default)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
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
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// This is so that we can do things like `total_force += force`
impl core::ops::AddAssign<Vec2> for Vec2 {
    fn add_assign(self: &mut Self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}