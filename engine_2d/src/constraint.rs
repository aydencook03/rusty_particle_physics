use crate::vec2::Vec2;

/// The type of constraint.
///
/// The `Equality` type means that the constraint is satisfied if function = 0.
/// The `Inequality` type means that the constraint is satisfied if function ≥ 0.
pub enum ConstraintKind {
    /// The constraint is satisfied if function = 0.
    Equality,
    /// The constraint is satisfied if function ≥ 0.
    Inequality,
}

pub struct Constraint {
    pub function: Box<dyn Fn(Vec2) -> f64>,
    pub stiffness: f64,
    pub kind: ConstraintKind,
    pub broken: bool,
}

impl Constraint {
    pub fn solver(_constraints: &[Constraint], pos: Vec2, iterations: u32) -> Vec2 {
        for _ in 0..iterations {}
        pos
    }
}
