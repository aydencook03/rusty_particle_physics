use crate::physics::particle::Particle;
use crate::vec2::Vec2;

/// The type of constraint.
///
/// The `Equality` type means that the constraint is satisfied if function = 0 (a Holonomic constraint).
/// The `Inequality` type means that the constraint is satisfied if function ≥ 0.
pub enum ConstraintKind {
    /// The constraint is satisfied if function = 0 (a Holonomic constraint).
    Equality,
    /// The constraint is satisfied if function ≥ 0.
    Inequality,
}

pub enum Constraint {
    /// A generalized, position based constraint.
    Constraint {
        particles: Vec<Particle>,
        function: Box<dyn Fn(Vec<Vec2>) -> f64>,
        compliance: f64,
        kind: ConstraintKind,
        broken: bool,
    },

    BoundingRect {},
}

impl Constraint {
    pub fn project(self: &Self, dt: f64) {
        match self {
            Constraint::Constraint { .. } => Constraint::solver(self, dt),
            _ => (),
        }
    }

    pub fn solver(_constraint: &Constraint, _dt: f64) {}
}
