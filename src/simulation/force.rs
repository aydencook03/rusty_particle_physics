use crate::math::Vec2;
use crate::particle::Particle;
use super::constraint::Constraint;

/// Force on a Particle or between interacting Particles
pub enum Force {
    /// A raw 2d force
    Raw { particle: &mut Particle, force: Vec2 },

    /// A general restoring force (F = -kx^n - bv) that attempts to satisfy a given constraint. Ex:
    /// - gravity {(Link)MaxDistance, G*mass*mass, -2, 0}
    /// - dampened spring between Particles {LinkFixedDistance, stiffness, 1, 0.5}
    /// - stiff pendulum {FixedDistance, 99, 1, 99}
    ConstraintForce { constraint_type: Constraint, k: f64, n: f64, b: f64 },

    /// A simple downwards pull of gravity (F = -mg)
    WorldGravity { particle: &mut Particle, g: f64 },

    /// Newtonian gravitational attraction between two Particles
    Gravity { particle1: &mut Particle, particle2: &mut Particle },

    /// Stokes drag force
    Drag { particle: &mut Particle, strength: f64 },
}

impl Force {
    /// Send the Force to the Particle(s)
    fn send_force(self: &Self) {
        todo!();
    }
}