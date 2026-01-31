use crate::constants::*;
use crate::qtree::Bound;
use crate::vector::Vec2;
use std::fmt;

pub struct Particle {
    pub prev_pos: Vec2,
    pub pos: Vec2,
    pub prev_acc: Vec2,
    pub acc: Vec2,
    pub mass: f32,
    pub radius: f32,
    pub n_collisions: u32,
    pub n_total_collisions: u32,
}

impl Particle {
    pub fn new() -> Particle {
        return Particle {
            prev_pos: Vec2::zero(),
            pos: Vec2::zero(),
            prev_acc: Vec2::zero(),
            acc: Vec2::zero(),
            mass: 1.0,
            radius: 0.0001,
            n_collisions: 0,
            n_total_collisions: 0,
        };
    }

    pub fn reset_collisions(&mut self) {
        self.n_collisions = 0;
        self.n_total_collisions = 0;
    }

    pub fn reset_acc(&mut self) {
        self.prev_acc = self.acc;
        self.acc = Vec2::zero();
    }

    pub fn set_density(&mut self, density: f32) {
        self.mass = self.radius * self.radius * self.radius * self.radius * density;
    }

    pub fn integrate(&mut self, dt: f32) {
        let mut vel = (self.pos - self.prev_pos);

        if vel.length_squared() > MAX_SPEED_SQUARED {
            vel = vel.normalize() * MAX_SPEED;
        }

        let new_pos = self.pos + vel + self.acc * (dt * dt);
        self.prev_pos = self.pos;
        self.pos = new_pos;
        self.reset_acc();
    }

    pub fn get_vel(&self) -> Vec2 {
        return self.pos - self.prev_pos;
    }

    pub fn get_speed(&self) -> f32 {
        let dx = self.pos.x - self.prev_pos.x;
        let dy = self.pos.y - self.prev_pos.y;

        return (dx * dx + dy * dy).sqrt();
    }

    pub fn set_pos(&mut self, new_pos: Vec2) {
        self.pos = new_pos;
        self.prev_pos = new_pos;
    }

    pub fn set_vel(&mut self, new_vel: Vec2) {
        self.prev_pos = self.pos - new_vel;
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.acc += force / self.mass;
    }

    pub fn get_bound(&self) -> Bound {
        let offset = Vec2::new(self.radius, self.radius) * 4.0;

        return Bound::new(self.pos - offset, self.pos + offset);
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

pub struct ParticleData {
    pub position: Vec2,
    pub speed: f32,
    pub n_collisions: u32,
}

impl ParticleData {
    pub fn new(particle: &Particle) -> ParticleData {
        return ParticleData {
            position: particle.pos,
            speed: particle.get_speed(),
            n_collisions: particle.n_total_collisions,
        };
    }
}
