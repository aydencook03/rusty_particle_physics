//! Provides the logic for particles and their external forces.
//!
//! The particles in this engine obey locality, which is why most of the interesting force logic is handled by the
//! global simulation. As this engine is non-field theoretic, there are no fields to locally mediate forces, and so
//! most interesting forces depend on all of the other particles at that instant. The global sim object is aware of all
//! particles at once, so it can calculate all of the non-local forces and then pass the force vectors to the particles as
//! a Vec2 that they can then handle.
//!
//! A particle is the most fundamental object in the physics engine, and can be used on its own if needed, as it
//! handles its own interactions with the outside world through forces.

use crate::vec2::Vec2;

pub const WHITE: (u8, u8, u8, u8) = (255, 255, 255, 255);
pub const BLACK: (u8, u8, u8, u8) = (0, 0, 0, 255);
pub const GREY: (u8, u8, u8, u8) = (40, 40, 40, 255);
pub const CRIMSON: (u8, u8, u8, u8) = (220, 20, 60, 255);
pub const EARTH_BLUE: (u8, u8, u8, u8) = (10, 30, 220, 255);
pub const FOREST_GREEN: (u8, u8, u8, u8) = (1, 79, 55, 255);

/// A physical particle.
#[derive(Default)]
pub struct Particle {
    /// mass of the particle
    pub mass: f64,
    /// 2-dimensional position of the particle
    pub pos: Vec2,
    /// 2-dimensional velocity of the particle
    pub vel: Vec2,
    /// a collection of all of the forces acting on the particle
    pub forces: Vec<Vec2>,
    /// the previous 2-dimensional position of the particle
    old_pos: Vec2,
    /// a field intended to be used as a unique ID for anytime that it is useful
    pub id: u32,
    /// when you want to group together particles with shared properties, etc
    pub group: u32,
    /// radius of the particle
    pub radius: f64,
    /// 32-bit color: (r, g, b, a)
    pub color: (u8, u8, u8, u8),
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

    /// A builder method to give the particle a specific id after creating it.
    pub fn id(mut self: Self, id: u32) -> Particle {
        self.id = id;
        self
    }

    /// A builder method to give the particle a specific group_num after creating it.
    pub fn group(mut self: Self, group: u32) -> Particle {
        self.group = group;
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
    /// Particle (uses the Semi-implicit/Symplectic Euler Method).
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
        let mut total_force = Vec2::zero();
        for force in &self.forces {
            total_force += *force;
        }
        self.vel += (total_force / self.mass) * dt;
        self.old_pos = self.pos;
        self.pos += self.vel * dt;
    }

    /// Update the velocity based on the previous position and the current position.
    ///
    /// This method exists so that an implicit velocity update doesn't need to occur in the `update` method, as
    /// something like this should be very explicit.
    ///
    /// It is mainly used for changing the velocity after some kind of non-physical constraint or
    /// position change has been applied.
    pub fn update_vel(self: &mut Self, dt: f64) {
        self.vel = (self.pos - self.old_pos) / dt;
    }
}
