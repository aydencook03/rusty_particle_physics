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

    StayInRect {
        particle: &'a mut Particle,
        top_left: Vec2,
        bottom_right: Vec2,
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
    pub fn handle(self: &mut Self) {
        match self {
            Constraint::StayInRect { particle, top_left, bottom_right } => {
                if particle.pos.x - particle.radius < top_left.x {
                    particle.pos.x = top_left.x + particle.radius;
                } else if particle.pos.x + particle.radius > bottom_right.x {
                    particle.pos.x = bottom_right.x - particle.radius;
                }
                if particle.pos.y - particle.radius < top_left.y {
                    particle.pos.y = top_left.y + particle.radius;
                } else if particle.pos.y + particle.radius > bottom_right.y {
                    particle.pos.y = bottom_right.y - particle.radius;
                }
            }
            _ => (),
        }
    }
}
