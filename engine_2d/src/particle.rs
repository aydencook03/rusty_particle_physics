//! Provides the Particle type and some colors.
//! 
//! A particle is the most fundamental object in the physics engine, and can be used on its own if needed, as it 
//! handles its own interactions with the outside world through forces.
//!  
//! It contains a set of physical properties such as mass, radius, color, etc.
//! 
//! It also contains the state of the particle, namely its Position and Velocity. 
//! As the scope of this engine is classical, non-relativistic, and non-field theoretic, the particle's state 
//! evolves according to the following simple rules:
//! 
//! External Forces update velocity.
//! Velocity updates position.
//! 
//! Therefore, the Particle::update method is explicit in velocity, and uses the Symplectic-Euler method.
//! 
//! Example usage (without using Sim):
//! 
//! ```rust
//! let mut particle = Particle::new()
//!     .radius(10.0)
//!     .pos(0.0, 0.0)
//!     .vel(50.0, 70.0)
//!     .color(EARTH_BLUE);
//! 
//! particle.forces.push(Vec2::new(0.0, -400.0));
//! particle.update(0.01);
//! println!("{}", particle.pos.x);
//! 
//! let mut particle2 = Particle::new();
//! particle2.vel = Vec2::new(7.0, 0.0);
//! ```

use crate::vec2::Vec2;

pub const WHITE: (u8, u8, u8, u8) = (255, 255, 255, 255);
pub const BLACK: (u8, u8, u8, u8) = (0, 0, 0, 255);
pub const GREY: (u8, u8, u8, u8) = (40, 40, 40, 255);
pub const CRIMSON: (u8, u8, u8, u8) = (220, 20, 60, 255);
pub const EARTH_BLUE: (u8, u8, u8, u8) = (10, 30, 220, 255);
pub const FOREST_GREEN: (u8, u8, u8, u8) = (1, 79, 55, 255);

/// A physical particle. Is only aware of its own properties, state, and the forces acting on it.
#[derive(Default, Clone)]
pub struct Particle {
    /// mass of the particle
    pub mass: f64,
    /// radius of the particle
    pub radius: f64,
    /// 32-bit color: (r, g, b, a)
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
    /// Constructor function for a default particle.
    pub fn new() -> Particle {
        Particle {
            mass: 10.0,
            radius: 10.0,
            color: CRIMSON,
            ..Default::default()
        }
    }

    /// A builder method to give the particle a specific mass after creating it.
    pub fn mass(mut self: Self, mass: f64) -> Particle {
        self.mass = mass;
        self
    }

    /// A builder method to give the particle a specific radius after creating it.
    pub fn radius(mut self: Self, radius: f64) -> Particle {
        self.radius = radius;
        self
    }

    /// A builder method to give the particle a specific color after creating it.
    pub fn color(mut self: Self, color: (u8, u8, u8, u8)) -> Particle {
        self.color = color;
        self
    }

    /// A builder method to give the particle a specific group_num after creating it.
    pub fn group_num(mut self: Self, group_num: u32) -> Particle {
        self.group_num = group_num;
        self
    }

    /// A builder method to give the particle a specific position after creating it.
    pub fn pos(mut self: Self, x: f64, y: f64) -> Particle {
        self.pos = Vec2::new(x, y);
        self
    }

    /// A builder method to give the particle a specific velocity after creating it.
    pub fn vel(mut self: Self, vel_x: f64, vel_y: f64) -> Particle {
        self.vel = Vec2::new(vel_x, vel_y);
        self
    }

    /// An explicit, first-order symplectic integrator that updates the 
    /// Particle (uses Semi-implicit/Symplectic Euler).
    ///
    /// A classical particle behaves according to:
    /// $$\frac{d}{dt}\begin{bmatrix}\vec{x} \\\ \vec{v}\end{bmatrix}=
    /// \begin{bmatrix}\vec{v} \\\ \frac{1}{m}\Sigma\vec{F}\end{bmatrix}$$
    ///
    /// This numerical integration scheme is a first-order symplectic integrator that solves this 
    /// differential equation using the following steps:
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
}
