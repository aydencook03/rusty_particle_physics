/// A 2d euclidean vector
#[derive(Copy, Clone, Default)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    /// Create a Vec2 using x and y components
    pub fn new(x: f64, y: f64) -> Vec2 {
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

/// So we can add vectors together
impl core::ops::AddAssign<Vec2> for Vec2 {
    fn add_assign(self: &mut Self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

/// So we can multiply vectors by a scalar
impl core::ops::Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self: Self, rhs: f64) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

/// So we can divide vectors by a scalar
impl core::ops::Div<f64> for Vec2 {
    type Output = Vec2;
    fn div(self: Self, rhs: f64) -> Self::Output {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}