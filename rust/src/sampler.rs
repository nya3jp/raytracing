use crate::geom::{Axis, IntoVec3, Vec3, Vec3Unit};
use crate::rng::Rng;
use itertools::Itertools;
use rand::prelude::SliceRandom;
use rand::Rng as _;
use std::f64::consts::PI;
use std::fmt::Debug;
use std::rc::Rc;
use std::sync::Arc;

pub trait Sampler: Debug {
    fn constant(&self) -> Option<Vec3Unit>;
    fn sample(&self, rng: &mut Rng) -> Vec3Unit;
    fn probability(&self, dir: Vec3Unit) -> f64;
}

impl Sampler for Box<dyn Sampler> {
    fn constant(&self) -> Option<Vec3Unit> {
        self.as_ref().constant()
    }
    fn sample(&self, rng: &mut Rng) -> Vec3Unit {
        self.as_ref().sample(rng)
    }
    fn probability(&self, dir: Vec3Unit) -> f64 {
        self.as_ref().probability(dir)
    }
}

impl Sampler for Rc<dyn Sampler> {
    fn constant(&self) -> Option<Vec3Unit> {
        self.as_ref().constant()
    }
    fn sample(&self, rng: &mut Rng) -> Vec3Unit {
        self.as_ref().sample(rng)
    }
    fn probability(&self, dir: Vec3Unit) -> f64 {
        self.as_ref().probability(dir)
    }
}

impl Sampler for Arc<dyn Sampler> {
    fn constant(&self) -> Option<Vec3Unit> {
        self.as_ref().constant()
    }
    fn sample(&self, rng: &mut Rng) -> Vec3Unit {
        self.as_ref().sample(rng)
    }
    fn probability(&self, dir: Vec3Unit) -> f64 {
        self.as_ref().probability(dir)
    }
}

#[derive(Debug)]
pub struct LambertianSampler {
    out_normal: Vec3Unit,
}

impl Sampler for LambertianSampler {
    fn constant(&self) -> Option<Vec3Unit> {
        None
    }

    fn sample(&self, rng: &mut Rng) -> Vec3Unit {
        (self.out_normal + Vec3::random_in_unit_sphere(rng)).unit()
    }

    fn probability(&self, dir: Vec3Unit) -> f64 {
        let cos = self.out_normal.dot(dir);
        if cos < 0.0 {
            0.0
        } else {
            cos / PI
        }
    }
}

impl LambertianSampler {
    pub fn new(out_normal: Vec3Unit) -> Self {
        LambertianSampler { out_normal }
    }
}

#[derive(Debug)]
pub struct RotateSampler {
    axis: Axis,
    theta: f64,
    sampler: Box<dyn Sampler>,
}

impl Sampler for RotateSampler {
    fn constant(&self) -> Option<Vec3Unit> {
        self.sampler.constant()
    }

    fn sample(&self, rng: &mut Rng) -> Vec3Unit {
        self.sampler
            .sample(rng)
            .rotate_around(self.axis, self.theta)
    }

    fn probability(&self, dir: Vec3Unit) -> f64 {
        self.sampler
            .probability(dir.rotate_around(self.axis, -self.theta))
    }
}

impl RotateSampler {
    pub fn new(axis: Axis, theta: f64, sampler: Box<dyn Sampler>) -> Self {
        RotateSampler {
            axis,
            theta,
            sampler,
        }
    }
}

#[derive(Debug)]
pub struct ConstantSampler {
    dir: Vec3Unit,
}

impl Sampler for ConstantSampler {
    fn constant(&self) -> Option<Vec3Unit> {
        Some(self.dir)
    }

    fn sample(&self, _rng: &mut Rng) -> Vec3Unit {
        self.dir
    }

    fn probability(&self, _dir: Vec3Unit) -> f64 {
        f64::INFINITY
    }
}

impl ConstantSampler {
    pub fn new(dir: Vec3Unit) -> Self {
        ConstantSampler { dir }
    }
}

#[derive(Debug)]
pub struct RectangleSampler {
    axis: Axis,
    a: f64,
    b_min: f64,
    b_max: f64,
    c_min: f64,
    c_max: f64,
}

impl Sampler for RectangleSampler {
    fn constant(&self) -> Option<Vec3Unit> {
        None
    }

    fn sample(&self, rng: &mut Rng) -> Vec3Unit {
        let b = rng.gen_range(self.b_min..=self.b_max);
        let c = rng.gen_range(self.c_min..=self.c_max);
        Vec3::new(self.a, b, c)
            .rotate_axes(Axis::X, self.axis)
            .unit()
    }

    fn probability(&self, dir: Vec3Unit) -> f64 {
        let dir = dir.rotate_axes(self.axis, Axis::X);
        if dir.x == 0.0 {
            return 0.0;
        }
        let t = self.a / dir.x;
        if t < 0.0 {
            return 0.0;
        }
        let b = dir.y * t;
        let c = dir.z * t;
        if b < self.b_min || b > self.b_max || c < self.c_min || c > self.c_max {
            return 0.0;
        }
        let l2 = Vec3::new(self.a, b, c).norm();
        l2 / (dir.x.abs() * (self.b_max - self.b_min) * (self.c_max - self.c_min))
    }
}

impl RectangleSampler {
    pub fn new(axis: Axis, a: f64, b_min: f64, b_max: f64, c_min: f64, c_max: f64) -> Self {
        RectangleSampler {
            axis,
            a,
            b_min,
            b_max,
            c_min,
            c_max,
        }
    }
}

#[derive(Debug)]
pub struct SphereSampler {
    center: Vec3,
    radius: f64,
}

impl Sampler for SphereSampler {
    fn constant(&self) -> Option<Vec3Unit> {
        if self.radius == 0.0 {
            Some(self.center.unit())
        } else {
            None
        }
    }

    fn sample(&self, rng: &mut Rng) -> Vec3Unit {
        (self.center + Vec3::random_in_unit_sphere(rng).unit() * self.radius).unit()
    }

    fn probability(&self, dir: Vec3Unit) -> f64 {
        let b2 = self.center.dot(dir);
        let d = b2 * b2 + self.radius * self.radius - self.center.norm();
        if d <= 0.0 {
            return 0.0;
        }
        let droot = d.sqrt();
        let t1 = b2 - droot;
        let t2 = b2 + droot;
        let a1 = if t1 > 0.0 {
            t1 * t1 / (dir * t1 - self.center).unit().dot(dir).abs()
        } else {
            0.0
        };
        let a2 = if t2 > 0.0 {
            t2 * t2 / (dir * t2 - self.center).unit().dot(dir).abs()
        } else {
            0.0
        };
        (a1 + a2) / (4.0 * PI * self.radius * self.radius)
    }
}

impl SphereSampler {
    pub fn new(center: Vec3, radius: f64) -> Self {
        SphereSampler { center, radius }
    }
}

#[derive(Debug)]
pub struct MixedSampler<S: Sampler> {
    samplers: Vec<S>,
}

impl<S: Sampler> Sampler for MixedSampler<S> {
    fn constant(&self) -> Option<Vec3Unit> {
        let v = self
            .samplers
            .iter()
            .filter_map(|sampler| sampler.constant())
            .collect_vec();
        match v.len() {
            0 => None,
            1 => Some(v[0]),
            _ => panic!("Cannot mix multiple constant samplers"),
        }
    }

    fn sample(&self, rng: &mut Rng) -> Vec3Unit {
        self.samplers.choose(rng).unwrap().sample(rng)
    }

    fn probability(&self, dir: Vec3Unit) -> f64 {
        self.samplers
            .iter()
            .map(|sampler| sampler.probability(dir))
            .sum::<f64>()
            / self.samplers.len() as f64
    }
}

impl<S: Sampler> MixedSampler<S> {
    pub fn new(samplers: impl IntoIterator<Item = S>) -> Self {
        MixedSampler {
            samplers: samplers.into_iter().collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    fn verify_sampler(sampler: impl Sampler) {
        let n = 10000000;
        let mut rng = Rng::seed_from_u64(28);
        let sum = (0..n)
            .map(|_| sampler.probability(Vec3::random_in_unit_sphere(&mut rng).unit()))
            .sum::<f64>();
        let integral = (sum / n as f64) * 4.0 * PI;
        eprintln!("Integral: {}", integral);
        assert!((1.0 - integral).abs() < 0.03);
    }

    #[test]
    fn test_rectangle_sampler() {
        verify_sampler(RectangleSampler::new(Axis::Y, 12.0, 33.0, 44.0, 60.0, 87.0));
    }

    #[test]
    fn test_lambertian_sampler() {
        verify_sampler(LambertianSampler::new(Vec3::new(1.0, 2.0, 3.0).unit()));
    }

    #[test]
    fn test_sphere_sampler() {
        verify_sampler(SphereSampler::new(Vec3::new(10.0, 20.0, 30.0), 5.7));
    }

    #[test]
    fn test_rotate_sampler() {
        verify_sampler(RotateSampler::new(
            Axis::Z,
            PI / 3.7,
            Box::new(RectangleSampler::new(Axis::Y, 12.0, 33.0, 44.0, 60.0, 87.0)),
        ));
    }

    #[test]
    fn test_mixed_sampler() {
        verify_sampler(MixedSampler::new(vec![
            Box::new(RectangleSampler::new(Axis::Y, 12.0, 33.0, 44.0, 60.0, 87.0))
                as Box<dyn Sampler>,
            Box::new(SphereSampler::new(Vec3::new(10.0, 20.0, 30.0), 5.7)) as Box<dyn Sampler>,
        ]));
    }
}
