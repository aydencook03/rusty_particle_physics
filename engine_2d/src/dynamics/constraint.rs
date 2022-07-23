use crate::particle::Particle;
use crate::vec2::Vec2;

/// Constraint on a Particle or between linked Particles.
/// This simply holds the data, and can be used as a static constraint or a constraint force elsewhere
pub enum Constraint<'a> {
    /// The fundamental type of constraint.
    ///
    /// The other Constraint types usually return a configured GeneralConstraint when being handled.
    GeneralConstraint {},

    /// Pin a Particle to a point
    PinToPoint {
        particle: &'a mut Particle,
        point: Vec2,
    },

    BoundaryLine {
        particle: &'a mut Particle,
        start: Vec2,
        end: Vec2,
    },

    FixedDistance {
        particle: &'a mut Particle,
        point: Vec2,
        dist: f64,
    },

    MinDistance {
        particle: &'a mut Particle,
        point: Vec2,
        dist: f64,
    },

    MaxDistance {
        particle: &'a mut Particle,
        point: Vec2,
        dist: f64,
    },

    LinkFixedDistance {
        particle1: &'a mut Particle,
        particle2: &'a mut Particle,
        dist: f64,
    },

    LinkMinDistance {
        particle1: &'a mut Particle,
        particle2: &'a mut Particle,
        dist: f64,
    },

    LinkMaxDistance {
        particle1: &'a mut Particle,
        particle2: &'a mut Particle,
        dist: f64,
    },
}

impl<'a> Constraint<'a> {
    /// Handle the Constraint statically
    pub fn handle(self: &Self) {
        todo!();
    }
}
