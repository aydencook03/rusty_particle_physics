use crate::vec2::Vec2;

/// A physical particle. Is only aware of its own properties, state, and the forces acting on it (obeys locality)
#[derive(Default)]
pub struct Particle {
    /// mass of the particle
    pub mass: f64,
    /// radius of the particle
    pub radius: f64,
    /// color: (r, g, b, alpha)
    pub color: (u8, u8, u8, u8),
    /// free number to use for things like group rendering, grouping together properties (liquids, solids), etc
    pub group_num: u8,
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
            color: (220, 20, 60, 1), //CRIMSON,
            ..Default::default()
        }
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
        // creates a zero Vec2 using the Default trait's default() function
        // equivalent to: let mut total_force: Vec2 = core::default::Default::default();
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
