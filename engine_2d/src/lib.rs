pub mod constraint;
pub mod particle;
pub mod rendering;
pub mod simulation;
pub mod vec2;

pub mod prelude {
    //! Things one should import to get up and going quickly.
    //!
    //! ```rust
    //! use rusty_particle_physics_2d::prelude::*;
    //! ```
    pub use crate::{
        constraint::{Constraint, ConstraintKind},
        particle::*,
        simulation::{sim::Sim, sim_constraint::SimConstraint, sim_force::SimForce},
        vec2::Vec2,
    };
}
