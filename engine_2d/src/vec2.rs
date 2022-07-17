/// A 2d euclidean vector
#[derive(Copy, Clone, Default)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    /// Create a Vec2 using x and y components
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x, y }
    }

    /// Create a Vec2 using polar components
    pub fn new_polar(r: f64, angle: f64) -> Vec2 {
        Vec2 {
            x: r * angle.cos(),
            y: r * angle.sin(),
        }
    }

    /// Dot product with another Vec2
    pub fn dot(self: &Self, other: &Vec2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    /// Returns the magnitude of the Vec2
    pub fn mag(self: &Self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl core::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;
    fn add(self: Self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl core::ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;
    fn sub(self: Self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

/// So we can add assign vectors together
impl core::ops::AddAssign<Vec2> for Vec2 {
    fn add_assign(self: &mut Self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl core::ops::SubAssign<Vec2> for Vec2 {
    fn sub_assign(self: &mut Self, rhs: Vec2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

/// So we can multiply vectors by a scalar f64
impl core::ops::Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self: Self, rhs: f64) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

/// So we can divide vectors by a scalar f64
impl core::ops::Div<f64> for Vec2 {
    type Output = Vec2;
    fn div(self: Self, rhs: f64) -> Self::Output {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl core::ops::MulAssign<f64> for Vec2 {
    fn mul_assign(self: &mut Self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl core::ops::DivAssign<f64> for Vec2 {
    fn div_assign(self: &mut Self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
    }
}
