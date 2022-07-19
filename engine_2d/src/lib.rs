pub mod dynamics;
pub mod matter;
pub mod rendering;
pub mod sim;
pub mod vec2;

pub mod prelude {
    pub use crate::{
        vec2::Vec2,
        matter::particle::Particle,
        dynamics::{force::Force, constraint::Constraint},
        sim::Sim,
    };
}
