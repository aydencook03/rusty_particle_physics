pub mod dynamics;
pub mod particle;
pub mod sim;
pub mod vec2;

pub mod prelude {
    //! Things one should import to get up and going quickly.
    //!
    //! ```rust
    //! use rusty_particle_physics_2d::prelude::*;
    //! ```
    pub use crate::{
        dynamics::{constraint::Constraint, force::Force},
        particle::*,
        sim::Sim,
        vec2::Vec2,
    };
}
