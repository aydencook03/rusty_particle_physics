# Rusty Particle Simulation

## Technical Scope
This is a classical, non-relativistic, non-field-theoretic, particle based physics engine.

But what does this mean exactly?
- The most fundamental object is a particle.
- These particles behave dynamically according to $$\frac{d}{dt}\begin{bmatrix}\vec{r} \\\ \vec{p}\end{bmatrix}=\begin{bmatrix}\vec{p}/m \\\ \Sigma\vec{F}\end{bmatrix}$$
- There is no massless particle (these are only permitted in a relativistic theory).
- Particles do not possess angular momentum (no intrinsic spin).
- Mass isn't a measure of rest energy, so it is conserved.
- Interactions are non-local, as there are no fields to mediate forces.

This boils it down to only three conservation laws:
- Linear momentum
- Energy
- Mass

Why particle based?
- 

## References:
- Youtube [playlist](https://youtube.com/playlist?list=PLvypLlLlZuNhcdtPKfQ25cpmhBuWWDZzR) for ideas
- Work of [Miles Macklin](http://blog.mmacklin.com/) and [Matthias Müller](https://matthias-research.github.io/pages/)
- Papers:
    - Macklin, Müller, Chentanez, Kim. 2014. [Link](https://doi.org/10.1145/2601097.2601152).<br />Unified particle physics for real-time applications.
    - Stam. 2009. [Link](https://www.autodesk.com/research/publications/nucleus).<br />Nucleus: Towards a Unified Dynamics Solver for Computer Graphics.
    - Müller, Heidelberger, Hennix, Ratcliff. 2007. [Link](https://doi.org/10.1016/j.jvcir.2007.01.005).<br />Position based dynamics.

## Simulation Methods:
- Forces (force based dynamics)
- Constraints (position based dynamics)
- Impulses (impulse based dynamics)
