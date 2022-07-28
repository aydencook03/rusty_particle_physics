/// Force on a Particle or between interacting Particles
pub enum Force {
    /// The fundamental type of 2d force.
    ///
    /// The other Force types usually return a configured GeneralForce or RawForce when being handled.
    InteractionForce {},

    /// The other type of fundamental 2d force. However, this one is less realistic than InteractionForce.
    /// This is because it is not an interaction between two particles, meaning there can be no reaction force.
    /// No reaction force means that total linear momentum will not be conserved, unlike in the real universe.
    RawForce {},

    /// A general restoring force (F = -kx^n - bv) that attempts to satisfy a given constraint.
    ///
    /// Examples:
    /// - gravity { (Link)MaxDistance, G*mass*mass, -2, 0 }
    /// - dampened spring between Particles { LinkFixedDistance, stiffness, 1, 0.5 }
    /// - stiff pendulum { FixedDistance, 99, 1, 99 }
    ConstraintForce {},

    /// A simple downwards pull of gravity (F = -mg)
    WorldGravity {},

    /// Newtonian gravitational attraction between two Particles
    Gravity {},

    /// Drag force. [Wikipedia](https://en.wikipedia.org/wiki/Drag_equation)
    Drag {},
}

impl Force {
    /// Send the calculated force to the Particle(s) as a Vec2
    pub fn send(self: &Self) {
        todo!();
    }
}
