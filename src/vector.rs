use rand::Rng;
use std::fmt;
use std::ops::*;

#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        return Vec2 { x, y };
    }

    pub fn zero() -> Vec2 {
        return Vec2::new(0.0, 0.0);
    }

    pub fn dot(&self, other: Vec2) -> f32 {
        return self.x * other.x + self.y * other.y;
    }

    pub fn perp(&self) -> Vec2 {
        return Vec2::new(-self.y, self.x);
    }

    pub fn length(&self) -> f32 {
        return (self.length_squared()).sqrt();
    }

    pub fn normalize(&self) -> Vec2 {
        let l = self.length();
        return *self / l;
    }

    pub fn length_squared(&self) -> f32 {
        return self.x * self.x + self.y * self.y;
    }

    pub fn rand_uniform() -> Vec2 {
        let mut rng_thing = rand::thread_rng();

        let x: f32 = rng_thing.gen_range(-1.0..1.0);
        let y: f32 = rng_thing.gen_range(-1.0..1.0);
        return Vec2::new(x, y);
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, other: f32) -> Vec2 {
        Vec2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, other: f32) -> Vec2 {
        Vec2 {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Vec2 {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
