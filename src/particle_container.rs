use crate::particle::*;
use crate::vector::*;
use crate::vector::*;
use crate::constants::*;

pub struct Container {
    pub particles: Vec<Particle>,
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

    pub fn container_collisions(&mut self, dt: f32){
        for particle in &mut self.particles{
            let particle_vel = particle.get_vel();

            if particle.pos.x - particle.radius < -1.0{
                particle.pos = Vec2::new(-1.0 + particle.radius, particle.pos.y);
                particle.set_vel(Vec2::new((particle_vel.x).abs(), particle_vel.y));
            }
            else if particle.pos.x + particle.radius > 1.0{
                particle.pos = Vec2::new(1.0 - particle.radius, particle.pos.y);
                particle.set_vel(Vec2::new(-(particle_vel.x).abs(), particle_vel.y));
            }

            if particle.pos.y - particle.radius < -1.0{
                particle.pos = Vec2::new(particle.pos.x, -1.0 + particle.radius);
                particle.set_vel(Vec2::new(particle_vel.x, (particle_vel.y).abs()));
            }
            else if particle.pos.y + particle.radius > 1.0{
                particle.pos = Vec2::new(particle.pos.x, 1.0 - particle.radius);
                particle.set_vel(Vec2::new(particle_vel.x, -(particle_vel.y).abs()));
            }
        }
    }

    pub fn apply_gravity(&mut self){
        for particle in &mut self.particles{
            particle.apply_force(Vec2::new(0.0, GRAVITY_CONST) * particle.mass);
        }
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
