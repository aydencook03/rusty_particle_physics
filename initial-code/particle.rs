/// A physical particle.  Is only aware of its own properties, state, and the forces acting on it (obeys locality)
#[derive(Default)]
struct Particle {
    mass: f64,
    //charge: f64,
    radius: f64,
    color: (u8, u8, u8),
    /// free number to use for things like group rendering, grouping together properties (liquids, solids), etc
    group_num: u8,
    pos: Vec2,
    vel: Vec2,
    forces: Vec<Vec2>,
}

impl Particle {
    /// Constructor function
    fn new() -> Particle {
        Particle {
            mass: 10.0,
            radius: 10.0,
            color: (220, 20, 60, 1), //CRIMSON,
            ..Default::default()
        }
    }

    /// A first-order symplectic integrator that updates the Particle (uses Semi-implicic/Symplectic Euler)
    fn symplectic_euler_update(self: &mut Self, dt: f64) {
        let mut accel = Vec2::new();
        for force in self.forces {
            accel += force / self.mass;
        }
        self.forces.clear(); // should I do this?
        self.vel += accel * dt;
        self.pos += self.vel * dt;
    }

    /// A second-order symplectic integrator that updates the Particle (uses Basic Störmer–Verlet)
    fn verlet_update(self: &mut Self, dt: f64) {
        todo!();
    }
}