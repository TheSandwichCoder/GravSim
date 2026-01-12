use crate::particle::*;
use crate::vector::*;

pub struct Container {
    particles: Vec<Particle>,
}

impl Container {
    pub fn new() -> Container {
        return Container {
            particles: Vec::new(),
        };
    }

    pub fn add_particle(&mut self) {
        self.particles.push(Particle::new());
    }

    pub fn integrate_particles(&mut self, dt: f32) {
        for particle in &mut self.particles {
            particle.integrate(dt);
        }
    }

    pub fn index(&mut self, i: usize) -> &mut Particle {
        return &mut self.particles[i];
    }

    pub fn show_particles(&self) {
        for particle in &self.particles {
            println!("{}\n", particle);
        }
    }
}
