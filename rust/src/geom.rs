use crate::rng::Rng;
use rand::Rng as _;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Self::Output {
        v * self
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, m: f64) -> Self::Output {
        Vec3 {
            x: self.x * m,
            y: self.y * m,
            z: self.z * m,
        }
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, m: f64) -> Self::Output {
        Vec3 {
            x: self.x / m,
            y: self.y / m,
            z: self.z / m,
        }
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn dot(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self, rhs: Self) -> Self {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn norm(self) -> f64 {
        self.dot(self)
    }

    pub fn abs(self) -> f64 {
        self.norm().sqrt()
    }

    pub fn unit(self) -> Self {
        self / self.abs()
    }

    pub fn random_in_unit_sphere(rng: &mut Rng) -> Self {
        loop {
            let x = rng.gen_range(-1.0..=1.0);
            let y = rng.gen_range(-1.0..=1.0);
            let z = rng.gen_range(-1.0..=1.0);
            let v = Vec3::new(x, y, z);
            if v.norm() <= 1.0 {
                return v;
            }
        }
    }

    pub fn random_in_unit_disc(rng: &mut Rng) -> Self {
        loop {
            let x = rng.gen_range(-1.0..=1.0);
            let y = rng.gen_range(-1.0..=1.0);
            let v = Vec3::new(x, y, 0.0);
            if v.norm() <= 1.0 {
                return v;
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Self {
        Ray { origin, dir }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.dir * t
    }
}
