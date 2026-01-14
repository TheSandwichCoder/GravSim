mod constants;
mod particle;
mod particle_container;
mod simulation;
mod vector;

use crate::particle::*;
use crate::particle_container::*;
use crate::simulation::*;
use crate::vector::*;

fn main() {
    let mut simulation_specs = SimulationSpecs::default();
    simulation_specs.set_framerate(30);

    let mut simulation = Simulation::construct(&simulation_specs);

    simulation.run();
    // let mut container = Container::new();
    // container.add_particle();

    // let p1 = container.index(0);
    // p1.apply_force(Vec2::new(1.0, 0.0));

    // container.show_particles();
    // container.integrate_particles(0.01);
    // container.show_particles();
    println!("SIM START");
}
