pub mod interaction;
pub mod physics;
pub mod rendering;
pub mod vec2;

pub mod prelude {
    //! Things one should import to get up and going quickly.
    //!
    //! ```rust
    //! use rusty_particle_physics_2d::prelude::*;
    //! ```
    pub use crate::{
        physics::constraint::{Constraint, ConstraintKind},
        physics::force::Force,
        physics::particle::*,
        physics::system::System,
        vec2::Vec2,
    };
}
