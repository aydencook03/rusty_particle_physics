pub mod dynamics;
pub mod matter;
pub mod sim;
pub mod vec2;

pub mod prelude {
    pub use crate::{
        dynamics::{constraint::Constraint, force::Force},
        matter::particle::*,
        sim::Sim,
        vec2::Vec2,
    };
}
