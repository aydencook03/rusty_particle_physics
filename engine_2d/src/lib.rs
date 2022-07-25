pub mod interaction;
pub mod particle;
pub mod rendering;
pub mod sim;
pub mod sim_dynamics;
pub mod vec2;

pub mod prelude {
    //! Things one should import to get up and going quickly.
    //!
    //! ```rust
    //! use rusty_particle_physics_2d::prelude::*;
    //! ```
    pub use crate::{
        particle::*,
        sim::Sim,
        //sim_dynamics::{constraint::Constraint, force::Force},
        vec2::Vec2,
    };
}
