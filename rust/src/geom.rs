use crate::rng::Rng;
use rand::Rng as _;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

pub trait IntoVec3: Sized {
    fn into_vec3(self) -> Vec3;

    fn get(self, axis: Axis) -> f64 {
        let v = self.into_vec3();
        match axis {
            Axis::X => v.x,
            Axis::Y => v.y,
            Axis::Z => v.z,
        }
    }

    fn dot(self, rhs: impl IntoVec3) -> f64 {
        let lhs = self.into_vec3();
        let rhs = rhs.into_vec3();
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }

    fn cross(self, rhs: impl IntoVec3) -> Vec3 {
        let lhs = self.into_vec3();
        let rhs = rhs.into_vec3();
        Vec3 {
            x: lhs.y * rhs.z - lhs.z * rhs.y,
            y: lhs.z * rhs.x - lhs.x * rhs.z,
            z: lhs.x * rhs.y - lhs.y * rhs.x,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl IntoVec3 for Vec3 {
    fn into_vec3(self) -> Vec3 {
        self
    }
}

impl<T: IntoVec3> std::ops::Add<T> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.into_vec3();
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: IntoVec3> std::ops::Sub<T> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: T) -> Self::Output {
        let rhs = rhs.into_vec3();
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

    pub fn norm(self) -> f64 {
        self.dot(self)
    }

    pub fn abs(self) -> f64 {
        self.norm().sqrt()
    }

    pub fn unit(self) -> Vec3Unit {
        let u = self / self.abs();
        Vec3Unit::new(u.x, u.y, u.z)
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

    pub fn rotate_axes(self, mut from: Axis, mut to: Axis) -> Self {
        while from != Axis::X {
            from = from.next();
            to = to.next();
        }
        match to {
            Axis::X => self,
            Axis::Y => Vec3::new(self.z, self.x, self.y),
            Axis::Z => Vec3::new(self.y, self.z, self.x),
        }
    }

    pub fn rotate_around(self, axis: Axis, theta: f64) -> Self {
        let r = self.rotate_axes(axis, Axis::X);
        let r = Vec3::new(
            r.x,
            r.y * theta.cos() - r.z * theta.sin(),
            r.y * theta.sin() + r.z * theta.cos(),
        );
        r.rotate_axes(Axis::X, axis)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vec3Unit {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl IntoVec3 for Vec3Unit {
    fn into_vec3(self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl<T: IntoVec3> std::ops::Add<T> for Vec3Unit {
    type Output = Vec3;
    fn add(self, rhs: T) -> Self::Output {
        let lhs = self.into_vec3();
        let rhs = rhs.into_vec3();
        Vec3 {
            x: lhs.x + rhs.x,
            y: lhs.y + rhs.y,
            z: lhs.z + rhs.z,
        }
    }
}

impl<T: IntoVec3> std::ops::Sub<T> for Vec3Unit {
    type Output = Vec3;
    fn sub(self, rhs: T) -> Self::Output {
        let lhs = self.into_vec3();
        let rhs = rhs.into_vec3();
        Vec3 {
            x: lhs.x - rhs.x,
            y: lhs.y - rhs.y,
            z: lhs.z - rhs.z,
        }
    }
}

impl std::ops::Mul<Vec3Unit> for f64 {
    type Output = Vec3;
    fn mul(self, v: Vec3Unit) -> Self::Output {
        v.into_vec3() * self
    }
}

impl std::ops::Mul<f64> for Vec3Unit {
    type Output = Vec3;
    fn mul(self, m: f64) -> Self::Output {
        self.into_vec3() * m
    }
}

impl std::ops::Div<f64> for Vec3Unit {
    type Output = Vec3;
    fn div(self, m: f64) -> Self::Output {
        self.into_vec3() / m
    }
}

impl std::ops::Neg for Vec3Unit {
    type Output = Vec3Unit;
    fn neg(self) -> Self::Output {
        Vec3Unit::new(-self.x, -self.y, -self.z)
    }
}

impl Vec3Unit {
    pub const X: Vec3Unit = Vec3Unit {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    pub const Y: Vec3Unit = Vec3Unit {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    pub const Z: Vec3Unit = Vec3Unit {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };

    fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3Unit { x, y, z }
    }

    pub fn random_on_unit_sphere(rng: &mut Rng) -> Self {
        Vec3::random_in_unit_sphere(rng).unit()
    }

    pub fn random_on_unit_hemisphere(n: Vec3Unit, rng: &mut Rng) -> Self {
        let r = Vec3Unit::random_on_unit_sphere(rng);
        let r = Vec3Unit {
            x: r.x,
            y: r.y,
            z: r.z.abs(),
        };
        let u = n
            .cross(if n.x.abs() > 0.9 {
                Vec3Unit::Y
            } else {
                Vec3Unit::X
            })
            .unit();
        let v = n.cross(u).unit();
        (u * r.x + v * r.y + n * r.z).unit()
    }

    pub fn rotate_axes(self, mut from: Axis, mut to: Axis) -> Self {
        while from != Axis::X {
            from = from.next();
            to = to.next();
        }
        match to {
            Axis::X => self,
            Axis::Y => Vec3Unit::new(self.z, self.x, self.y),
            Axis::Z => Vec3Unit::new(self.y, self.z, self.x),
        }
    }

    pub fn rotate_around(self, axis: Axis, theta: f64) -> Self {
        let r = self.rotate_axes(axis, Axis::X);
        let r = Vec3Unit::new(
            r.x,
            r.y * theta.cos() - r.z * theta.sin(),
            r.y * theta.sin() + r.z * theta.cos(),
        );
        r.rotate_axes(Axis::X, axis)
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

    pub fn is_empty(self) -> bool {
        self.min.x >= self.max.x || self.min.y >= self.max.y || self.min.z >= self.max.z
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

    pub fn translate(self, offset: Vec3) -> Self {
        Box3::new(self.min + offset, self.max + offset)
    }

    pub fn iter_vertex(&self) -> Box3VertexIter {
        Box3VertexIter { bb: self, i: 0 }
    }
}

pub struct Box3VertexIter<'a> {
    bb: &'a Box3,
    i: usize,
}

impl<'a> Iterator for Box3VertexIter<'a> {
    type Item = Vec3;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == 8 {
            return None;
        }
        let bb = self.bb;
        let x = (if self.i & 1 != 0 { bb.max } else { bb.min }).x;
        let y = (if self.i & 2 != 0 { bb.max } else { bb.min }).y;
        let z = (if self.i & 4 != 0 { bb.max } else { bb.min }).z;
        self.i += 1;
        Some(Vec3::new(x, y, z))
    }
}
