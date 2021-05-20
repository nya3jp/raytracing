use rand::Rng as _;

use crate::rng::Rng;

#[derive(Clone, Copy, Debug)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    pub const ALL: [Axis; 3] = [Axis::X, Axis::Y, Axis::Z];

    pub fn next(self) -> Self {
        match self {
            Axis::X => Axis::Y,
            Axis::Y => Axis::Z,
            Axis::Z => Axis::X,
        }
    }
}

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
    pub const INFINITY: Vec3 = Vec3 {
        x: f64::INFINITY,
        y: f64::INFINITY,
        z: f64::INFINITY,
    };

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn get(self, axis: Axis) -> f64 {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
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

#[derive(Clone, Copy, Debug)]
pub struct Box3 {
    pub min: Vec3,
    pub max: Vec3,
}

impl Box3 {
    pub const EMPTY: Box3 = Box3 {
        min: Vec3 {
            x: f64::INFINITY,
            y: f64::INFINITY,
            z: f64::INFINITY,
        },
        max: Vec3 {
            x: -f64::INFINITY,
            y: -f64::INFINITY,
            z: -f64::INFINITY,
        },
    };

    pub fn new(min: Vec3, max: Vec3) -> Self {
        Box3 { min, max }
    }

    pub fn union(self, other: Self) -> Self {
        Box3 {
            min: Vec3::new(
                self.min.x.min(other.min.x),
                self.min.y.min(other.min.y),
                self.min.z.min(other.min.z),
            ),
            max: Vec3::new(
                self.max.x.max(other.max.x),
                self.max.y.max(other.max.y),
                self.max.z.max(other.max.z),
            ),
        }
    }
}
