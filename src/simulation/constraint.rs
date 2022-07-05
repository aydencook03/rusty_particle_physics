use crate::math::Vec2;
use crate::particle::Particle;

/// Constraint on a Particle or between linked Particles.
/// This simply holds the data, and can be used as a static constraint or a constraint force elsewhere
pub enum Constraint {
    /// Pin a Particle to a point
    PinToPoint { particle: &mut Particle, point: Vec2 },
    BoundaryLine { particle: &mut Particle, start: Vec2, end: Vec2 },
    FixedDistance { particle: &mut Particle, point: Vec2, dist: f64 },
    MinDistance { particle: &mut Particle, point: Vec2, dist: f64 },
    MaxDistance { particle: &mut Particle, point: Vec2, dist: f64 },

    LinkFixedDistance { particle1: &mut Particle, particle2: &mut Particle, dist: f64 },
    LinkMinDistance { particle1: &mut Particle, particle2: &mut Particle, dist: f64 },
    LinkMaxDistance { particle1: &mut Particle, particle2: &mut Particle, dist: f64 },
}

impl Constraint {
    /// Handle the Constraint statically
    fn handle_as_static_constraint(self: &Self) {
        match self {
            PinToPoint => {
                /* const is_satisfied: bool = EXPR...;
                if !is_satisfied {
                    handle...
                } */
            }
        }
    }
}