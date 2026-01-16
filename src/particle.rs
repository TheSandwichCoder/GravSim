use crate::constants::*;
use crate::vector::Vec2;
use std::fmt;

pub struct Particle {
    pub prev_pos: Vec2,
    pub pos: Vec2,
    pub prev_acc: Vec2,
    pub acc: Vec2,
    pub mass: f32,
    pub radius: f32,
}

impl Particle {
    pub fn new() -> Particle {
        return Particle {
            prev_pos: Vec2::zero(),
            pos: Vec2::zero(),
            prev_acc: Vec2::zero(),
            acc: Vec2::zero(),
            mass: 1.0,
            radius: 0.001,
        };
    }

    pub fn reset_acc(&mut self) {
        self.prev_acc = self.acc;
        self.acc = Vec2::zero();
    }

    pub fn integrate(&mut self, dt: f32) {
        let vel = (self.pos - self.prev_pos) * DAMPING;
        let new_pos = self.pos + vel + self.acc * (dt * dt);
        self.prev_pos = self.pos;
        self.pos = new_pos;
        self.reset_acc();
    }

    pub fn get_vel(&self) -> Vec2 {
        return self.pos - self.prev_pos;
    }

    pub fn set_vel(&mut self, new_vel: Vec2) {
        self.prev_pos = self.pos - new_vel;
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.acc += force / self.mass;
    }
}

// vel = (self.pos - self.prev_pos) * DAMPING
// new_pos = self.pos + vel + self.acc * (dt * dt)
// self.prev_pos = self.pos
// self.pos = new_pos
// self.acc.update(0, 0)

impl fmt::Display for Particle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Particle (pos: {} vel: {} acc: {})",
            self.pos,
            self.get_vel(),
            self.prev_acc
        )
    }
}

pub struct ParticleData{
    pub position: Vec2,
}

impl ParticleData{
    pub fn new(particle: &Particle) -> ParticleData{
        return ParticleData{
            position: particle.pos,
        };
    }
}