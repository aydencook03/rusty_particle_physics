use super::constraint::Constraint;
use crate::vec2::Vec2;
use crate::matter::particle::Particle;

/// Force on a Particle or between interacting Particles
pub enum Force {
    /// A raw 2d force
    Raw {
        particle: &mut Particle,
        force: Vec2,
    },

    /// A general restoring force (F = -kx^n - bv) that attempts to satisfy a given constraint.
    /// 
    /// Examples:
    /// - gravity { (Link)MaxDistance, G*mass*mass, -2, 0 }
    /// - dampened spring between Particles { LinkFixedDistance, stiffness, 1, 0.5 }
    /// - stiff pendulum { FixedDistance, 99, 1, 99 }
    ConstraintForce {
        constraint_type: Constraint,
        k: f64,
        n: f64,
        b: f64,
    },

    /// A simple downwards pull of gravity (F = -mg)
    WorldGravity { particle: &mut Particle, g: f64 },

    /// Newtonian gravitational attraction between two Particles
    Gravity {
        particle1: &mut Particle,
        particle2: &mut Particle,
        G: f64,
    },

    /// Drag force. [Wikipedia](https://en.wikipedia.org/wiki/Drag_equation)
    Drag {
        particle: &mut Particle,
        strength: f64,
    },
}

impl Force {
    /// Send the Force to the Particle(s)
    fn send_force(self: &Self) {
        match self {
            Force::Raw { particle, force } => particle.forces.push(force),
            Force::ConstraintForce { .. } => todo!(),
            Force::WorldGravity { particle, g } => particle.forces.push(Vec2 {
                x: 0.0,
                y: -particle.mass * g,
            }),
            Force::Gravity {
                particle1,
                particle2,
                G,
            } => todo!(),
            Force::Drag { particle, strength } => todo!(),
        }
    }
}
