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

/// A position based constraint.
pub struct Constraint {
    pub function: Box<dyn Fn(Vec2) -> f64>,
    pub compliance: f64,
    pub kind: ConstraintKind,
    pub broken: bool,
}

impl Constraint {
    pub fn solver(_constraint: &Constraint, _dt: f64) {}
}
