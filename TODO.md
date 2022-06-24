
# TODO / IDEAS

##### Things to figure out
- update velocity after constraint projection based on displacement & dt? (otherwise objects will continue to collide, etc)
- how does mass factor in? move each a certain fraction of the total distance weighted by mass?
- constraint projection must preserve conserved quantities. do forces and impulses preserve them?
- delete the forces after adding them to accel in Particle.update?

##### Tasks
- [ ] organize project
    - what is the project structure of egui?
    - [X] new git repo
    - [X] add as git submodule to programming repo
    - [ ] new cargo workspace
    - [ ] engine2d as sub cargo lib
    - [ ] engine3d as sub cargo lib
    - [ ] example_configs2d/3d folders
    - [ ] examples folder
    - [ ] create a good README
    - [ ] each renderer as sub cargo lib
        - Renderers
            - web_canvas_particle
            - eframe_particle
            - wgpu_solid -> treat groups of particles as object vertex
            - etc
- [X] add group_num to Particle (free number to use for things like render method, "phase type", etc)
- [ ] figure out good Sim encapsulation
- [ ] overload operators on Vec2
- [ ] run, step_sim, play_pause_toggle
- [ ] create a general type of constraint 
    - forces have a general variant (Raw) as well as specifics, why not same for constraints?
    - raw force uses Part,x,y.  What will general constraint use?
- [ ] add ability for constraints to break
    - add is_broken bool
    - should work in static & in force constraints
    - add breaking distance (0 if none)
    - in match statement, if over distance -> is_broken = true
    - remove constraint if broken: something like this,
        ```rust
        fn handle_static_constraints(self: &mut Self) {
            for constraint in self.static_constraints {
                if constraint.is_broken {
                    remove;
                } else {
                    constraint.handle_statically();
                }
            }
        }
        ```
- [ ] implement a rendering plugin api
    - rendering can be done independently. ie, render a solid rigid/soft body, fluid, etc
    - implement a rendering plugin api
    - send a packet of particle coordinates+radii & constraints/forces to be drawn?
    - this can be used to allow for both real-time and baked rendering
- [ ] save/load feature API -> serde the Sim struct -> file
    -   ```rust
        fn save(file: Option<?>) {
            match file {
                None => save in memory
                Some(file) => save to file
            }
            if let file = Some(?) {
                save to ?
            }
        }
        ```
- [ ] viscosity how? want this engine to be general enough for fluid sim
- [ ] methods for soft bodies? pneumatic, etc
- [ ] 3d version of the engine
- [ ] load 3d files as set of connected particles