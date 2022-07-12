use crate::vec2::Vec2;

/// A physical particle. Is only aware of its own properties, state, and the forces acting on it (obeys locality)
#[derive(Default)]
pub struct Particle {
    /// mass of the particle
    mass: f64,
    radius: f64,
    /// color: (r, g, b, alpha)
    color: (u8, u8, u8, u8),
    /// free number to use for things like group rendering, grouping together properties (liquids, solids), etc
    group_num: u8,
    /// 2-dimensional position of particle
    pos: Vec2,
    /// 2-dimensional velocity of particle
    vel: Vec2,
    /// a collection of all of the forces acting on the particle
    forces: Vec<Vec2>,
}

impl Particle {
    /// Constructor function for a default particle
    fn new() -> Particle {
        Particle {
            mass: 10.0,
            radius: 10.0,
            color: (220, 20, 60, 1), //CRIMSON,
            ..Default::default()
        }
    }

    /// A first-order symplectic integrator that updates the Particle (uses Semi-implicic/Symplectic Euler)
    fn symplectic_euler_update(self: &mut Self, dt: f64) {
        let mut accel = Vec2::new();
        for force in self.forces {
            accel += force / self.mass;
        }
        self.forces.clear(); // should I do this?
        self.vel += accel * dt;
        self.pos += self.vel * dt;
    }

    /// A second-order symplectic integrator that updates the Particle (uses ?)
    fn verlet_update(self: &mut Self, dt: f64) {
        todo!();
    }
}
