mod constants;
mod particle;
mod particle_container;
mod vector;

use crate::particle::*;
use crate::particle_container::*;
use crate::vector::*;

fn main() {
    let mut container = Container::new();
    container.add_particle();

    let mut p1 = container.index(0);
    p1.apply_force(Vec2::new(1.0, 0.0));

    container.show_particles();
    container.integrate_particles(0.01);
    container.show_particles();
    println!("SIM START");
}
