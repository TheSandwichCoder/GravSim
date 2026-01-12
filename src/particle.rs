use crate::constants::*;
use crate::vector::Vec2;
use std::fmt;

pub struct Particle {
    prev_pos: Vec2,
    pos: Vec2,
    vel: Vec2,
    acc: Vec2,
    mass: f32,
    radius: f32,
}

impl Particle {
    pub fn new() -> Particle {
        return Particle {
            prev_pos: Vec2::zero(),
            pos: Vec2::zero(),
            vel: Vec2::zero(),
            acc: Vec2::zero(),
            mass: 1.0,
            radius: 1.0,
        };
    }

    pub fn integrate(&mut self, dt: f32) {
        self.vel = (self.pos - self.prev_pos) * DAMPING;
        let new_pos = self.pos + self.vel + self.acc * (dt * dt);
        self.prev_pos = self.pos;
        self.pos = new_pos;
        self.acc = Vec2::zero();
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
            self.pos, self.vel, self.acc
        )
    }
}
