use crate::vec2::Vec2;
use crate::matter::particle::Particle;
use super::constraint::Constraint;

/// Force on a Particle or between interacting Particles
pub enum Force<'a> {
    /// The fundamental type of 2d force
    ///
    /// The other Force types usually return a configured GeneralForce when being handled
    GeneralForce {
        particle1: Option<&'a mut Particle>,
        particle2: &'a Particle,
        force: Vec2,
    },

    /// A general restoring force (F = -kx^n - bv) that attempts to satisfy a given constraint.
    ///
    /// Examples:
    /// - gravity { (Link)MaxDistance, G*mass*mass, -2, 0 }
    /// - dampened spring between Particles { LinkFixedDistance, stiffness, 1, 0.5 }
    /// - stiff pendulum { FixedDistance, 99, 1, 99 }
    ConstraintForce {
        constraint_type: Constraint<'a>,
        k: f64,
        n: f64,
        b: f64,
    },

    /// A simple downwards pull of gravity (F = -mg)
    WorldGravity { particle: &'a mut Particle, g: f64 },

    /// Newtonian gravitational attraction between two Particles
    Gravity {
        particle1: &'a mut Particle,
        particle2: &'a mut Particle,
        g: f64,
    },

    /// Drag force. [Wikipedia](https://en.wikipedia.org/wiki/Drag_equation)
    Drag {
        particle: &'a mut Particle,
        strength: f64,
    },
}

impl<'a> Force<'a> {
    /// Send the Force to the Particle(s)
    pub fn send_force(self: &Self) {
        todo!();
    }
}
