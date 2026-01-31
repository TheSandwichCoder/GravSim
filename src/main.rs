mod constants;
mod functions;
mod particle;
mod particle_container;
mod progress_bar;
mod qtree;
mod simulation;
mod vector;

use std::io::Sink;

use crate::functions::*;
use crate::particle::*;
use crate::particle_container::*;
use crate::progress_bar::*;
use crate::qtree::*;
use crate::simulation::*;
use crate::vector::*;

fn main() {
    let mut simulation_specs = SimulationSpecs::default();
    simulation_specs.set_framerate(30);
    simulation_specs.set_sim_time(60.0);
    simulation_specs.set_recording(true);
    simulation_specs.set_n_particles(50000);
    simulation_specs.set_distribution(NORMAL_DISTRIBUTION);
    simulation_specs.set_spawn_radius(0.4);
    simulation_specs.set_n_collision_steps(8);
    simulation_specs.set_update_cache_steps(4);

    simulation_specs.set_n_sub_steps(32);

    let mut simulation = Simulation::construct(&simulation_specs);

    simulation.run();
    simulation
        .sim_recorder
        .export_recording("simulations/simulation_output.txt");
}
