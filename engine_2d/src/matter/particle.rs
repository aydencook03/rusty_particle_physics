use crate::vec2::Vec2;

/// A physical particle. Is only aware of its own properties, state, and the forces acting on it (obeys locality)
#[derive(Default, Clone)]
pub struct Particle {
    /// mass of the particle
    pub mass: f64,
    /// radius of the particle
    pub radius: f64,
    /// 32-bit color: (r, g, b, alpha)
    pub color: (u8, u8, u8, u8),
    /// free number to use for things like group rendering, grouping together properties (liquids, solids), etc
    pub group_num: u32,
    /// 2-dimensional position of the particle
    pub pos: Vec2,
    /// 2-dimensional velocity of the particle
    pub vel: Vec2,
    /// a collection of all of the forces acting on the particle
    pub forces: Vec<Vec2>,
}

impl Particle {
    /// Constructor function for a default particle
    pub fn new() -> Particle {
        Particle {
            mass: 10.0,
            radius: 10.0,
            color: (220, 20, 60, 255), //CRIMSON,
            ..Default::default()
        }
    }

    /// a builder method to give the particle a specific mass after creating it
    pub fn mass(mut self: Self, mass: f64) -> Particle {
        self.mass = mass;
        self
    }

    /// a builder method to give the particle a specific radius after creating it
    pub fn radius(mut self: Self, radius: f64) -> Particle {
        self.radius = radius;
        self
    }

    /// a builder method to give the particle a specific color after creating it
    pub fn color(mut self: Self, color: (u8, u8, u8, u8)) -> Particle {
        self.color = color;
        self
    }

    /// a builder method to give the particle a specific group_num after creating it
    pub fn group_num(mut self: Self, group_num: u32) -> Particle {
        self.group_num = group_num;
        self
    }

    /// a builder method to give the particle a specific position after creating it
    pub fn pos(mut self: Self, x: f64, y: f64) -> Particle {
        self.pos = Vec2::new(x, y);
        self
    }

    /// a builder method to give the particle a specific velocity after creating it
    pub fn vel(mut self: Self, vel_x: f64, vel_y: f64) -> Particle {
        self.vel = Vec2::new(vel_x, vel_y);
        self
    }

    /// An explicit, first-order symplectic integrator that updates the Particle (uses Semi-implicit/Symplectic Euler).
    ///
    /// A classical particle behaves according to:
    /// $$\frac{d}{dt}\begin{bmatrix}\vec{x} \\\ \vec{v}\end{bmatrix}=\begin{bmatrix}\vec{v} \\\ \frac{1}{m}\Sigma\vec{F}\end{bmatrix}$$
    ///
    /// This numerical integration scheme is a first-order symplectic integrator that solves this differential equation using the following steps:
    /// $$\vec{v} _{n+1} = \vec{v} _{n} + \frac{1}{m}\Sigma\vec{F} _{n}\Delta t$$
    /// $$\vec{x} _{n+1} = \vec{x} _{n} + \vec{v} _{n+1}\Delta t$$
    pub fn update(self: &mut Self, dt: f64) {
        let mut total_force = Vec2::new(0.0, 0.0);
        for force in &mut self.forces {
            total_force += *force;
        }
        self.vel += (total_force / self.mass) * dt;
        self.pos += self.vel * dt;
    }

    /// A second-order symplectic integrator that updates the Particle (uses ?)
    pub fn verlet_update(self: &mut Self, dt: f64) {
        todo!("{}", dt);
    }
}
