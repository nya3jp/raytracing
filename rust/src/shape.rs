use crate::geom::{Axis, Box3, IntoVec3, Vec3, Vec3Unit};
use crate::ray::Ray;
use crate::sampler::{MixedSampler, RectangleSampler, RotateSampler, Sampler, SphereSampler};
use crate::time::TimeRange;
use itertools::Itertools;
use std::f64::consts::PI;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct Hit {
    pub point: Vec3,
    pub normal: Vec3Unit,
    pub t: f64,
    pub u: f64,
    pub v: f64,
}

pub trait Shape: Debug + Sync + Send {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
    fn bounding_box(&self, time: TimeRange) -> Box3;
    fn sampler(&self, from: Vec3, time: f64) -> Option<Box<dyn Sampler>>;
    fn is_empty(&self) -> bool;
}

impl Shape for Box<dyn Shape> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        self.as_ref().hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, time: TimeRange) -> Box3 {
        self.as_ref().bounding_box(time)
    }

    fn sampler(&self, from: Vec3, time: f64) -> Option<Box<dyn Sampler>> {
        self.as_ref().sampler(from, time)
    }

    fn is_empty(&self) -> bool {
        self.as_ref().is_empty()
    }
}

#[derive(Clone, Debug)]
pub struct Empty {}

pub const EMPTY_SHAPE: Empty = Empty {};

impl Shape for Empty {
    fn hit(&self, _ray: &Ray, _t_min: f64, _t_max: f64) -> Option<Hit> {
        None
    }

    fn bounding_box(&self, _time: TimeRange) -> Box3 {
        Box3::EMPTY
    }

    fn sampler(&self, _from: Vec3, _time: f64) -> Option<Box<dyn Sampler>> {
        None
    }

    fn is_empty(&self) -> bool {
        true
    }
}

#[derive(Clone, Debug)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Shape for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let b2 = ray.dir.dot(oc);
        let c = oc.norm() - self.radius * self.radius;
        let discriminant = b2 * b2 - c;
        if discriminant < 0.0 {
            return None;
        }

        let droot = discriminant.sqrt();
        let t = {
            let t_lo = -b2 - droot;
            if t_min <= t_lo && t_lo <= t_max {
                t_lo
            } else {
                let t_hi = -b2 + droot;
                if t_min <= t_hi && t_hi <= t_max {
                    t_hi
                } else {
                    return None;
                }
            }
        };

        let point = ray.at(t);
        let normal = (point - self.center).unit();
        let theta = (-normal.y).acos();
        let phi = f64::atan2(-normal.z, normal.x) + PI;
        let u = phi / (2.0 * PI);
        let v = theta / PI;
        Some(Hit {
            point,
            normal,
            t,
            u,
            v,
        })
    }

    fn bounding_box(&self, _time: TimeRange) -> Box3 {
        let r = Vec3::new(self.radius, self.radius, self.radius);
        Box3::new(self.center - r, self.center + r)
    }

    fn sampler(&self, from: Vec3, _time: f64) -> Option<Box<dyn Sampler>> {
        Some(Box::new(SphereSampler::new(
            self.center - from,
            self.radius,
        )))
    }

    fn is_empty(&self) -> bool {
        self.radius == 0.0
    }
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

#[derive(Clone, Debug)]
pub struct MovingSphere {
    center0: Vec3,
    center1: Vec3,
    time: TimeRange,
    radius: f64,
}

impl Shape for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let center = self.center_at(ray.time);
        let oc = ray.origin - center;
        let b2 = ray.dir.dot(oc);
        let c = oc.norm() - self.radius * self.radius;
        let discriminant = b2 * b2 - c;
        if discriminant < 0.0 {
            return None;
        }

        let droot = discriminant.sqrt();
        let t = {
            let t_lo = -b2 - droot;
            if t_min <= t_lo && t_lo <= t_max {
                t_lo
            } else {
                let t_hi = -b2 + droot;
                if t_min <= t_hi && t_hi <= t_max {
                    t_hi
                } else {
                    return None;
                }
            }
        };

        let point = ray.at(t);
        let normal = (point - center).unit();
        let theta = (-normal.y).acos();
        let phi = f64::atan2(-normal.z, normal.x) + PI;
        let u = phi / (2.0 * PI);
        let v = theta / PI;
        Some(Hit {
            point,
            normal,
            t,
            u,
            v,
        })
    }

    fn bounding_box(&self, time: TimeRange) -> Box3 {
        let center0 = self.center_at(time.lo);
        let center1 = self.center_at(time.hi);
        let r = Vec3::new(self.radius, self.radius, self.radius);
        Box3::new(center0 - r, center0 + r).union(Box3::new(center1 - r, center1 + r))
    }

    fn sampler(&self, from: Vec3, time: f64) -> Option<Box<dyn Sampler>> {
        Some(Box::new(SphereSampler::new(
            self.center_at(time) - from,
            self.radius,
        )))
    }

    fn is_empty(&self) -> bool {
        self.radius == 0.0
    }
}

impl MovingSphere {
    pub fn new(center0: Vec3, center1: Vec3, time: TimeRange, radius: f64) -> Self {
        MovingSphere {
            center0,
            center1,
            time,
            radius,
        }
    }

    fn center_at(&self, time: f64) -> Vec3 {
        self.center0
            + (time - self.time.lo) * (self.time.hi - self.time.lo) * (self.center1 - self.center0)
    }
}

#[derive(Clone, Debug)]
pub struct Rectangle {
    axis: Axis,
    a: f64,
    b_min: f64,
    b_max: f64,
    c_min: f64,
    c_max: f64,
}

impl Shape for Rectangle {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let t = (self.a - ray.origin.get(self.axis)) / ray.dir.get(self.axis);
        if t.is_nan() || t < t_min || t > t_max {
            return None;
        }
        let point = ray.at(t);
        let b = point.get(self.axis.next());
        let c = point.get(self.axis.next().next());
        if b < self.b_min || b > self.b_max || c < self.c_min || c > self.c_max {
            return None;
        }
        let normal = Vec3Unit::X.rotate_axes(Axis::X, self.axis);
        let u = (b - self.b_min) / (self.b_max - self.b_min);
        let v = (c - self.c_min) / (self.c_max - self.c_min);
        Some(Hit {
            point,
            normal,
            t,
            u,
            v,
        })
    }

    fn bounding_box(&self, _time: TimeRange) -> Box3 {
        Box3::new(
            Vec3::new(self.a, self.b_min, self.c_min).rotate_axes(Axis::X, self.axis),
            Vec3::new(self.a, self.b_max, self.c_max).rotate_axes(Axis::X, self.axis),
        )
    }

    fn sampler(&self, from: Vec3, _time: f64) -> Option<Box<dyn Sampler>> {
        if self.b_min >= self.b_max || self.c_min >= self.c_max {
            None
        } else {
            let o = from.rotate_axes(self.axis, Axis::X);
            Some(Box::new(RectangleSampler::new(
                self.axis,
                self.a - o.x,
                self.b_min - o.y,
                self.b_max - o.y,
                self.c_min - o.z,
                self.c_max - o.z,
            )))
        }
    }

    fn is_empty(&self) -> bool {
        self.b_min >= self.b_max || self.c_min >= self.c_max
    }
}

impl Rectangle {
    pub fn new(axis: Axis, a: f64, b_min: f64, b_max: f64, c_min: f64, c_max: f64) -> Rectangle {
        Rectangle {
            axis,
            a,
            b_min,
            b_max,
            c_min,
            c_max,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Block {
    bb: Box3,
    union: Union<Rectangle>,
}

impl Shape for Block {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        self.union.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, _time: TimeRange) -> Box3 {
        self.bb
    }

    fn sampler(&self, from: Vec3, time: f64) -> Option<Box<dyn Sampler>> {
        self.union.sampler(from, time)
    }

    fn is_empty(&self) -> bool {
        self.bb.is_empty()
    }
}

impl Block {
    pub fn new(bb: Box3) -> Block {
        Block {
            bb,
            union: Union::new(vec![
                Rectangle::new(Axis::X, bb.min.x, bb.min.y, bb.max.y, bb.min.z, bb.max.z),
                Rectangle::new(Axis::X, bb.max.x, bb.min.y, bb.max.y, bb.min.z, bb.max.z),
                Rectangle::new(Axis::Y, bb.min.y, bb.min.z, bb.max.z, bb.min.x, bb.max.x),
                Rectangle::new(Axis::Y, bb.max.y, bb.min.z, bb.max.z, bb.min.x, bb.max.x),
                Rectangle::new(Axis::Z, bb.min.z, bb.min.x, bb.max.x, bb.min.y, bb.max.y),
                Rectangle::new(Axis::Z, bb.max.z, bb.min.x, bb.max.x, bb.min.y, bb.max.y),
            ]),
        }
    }
}

#[derive(Debug)]
pub struct Translate<S: Shape> {
    offset: Vec3,
    shape: S,
}

impl<S: Shape> Shape for Translate<S> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let ray = Ray::new(ray.origin - self.offset, ray.dir, ray.time);
        self.shape.hit(&ray, t_min, t_max).map(|hit| Hit {
            point: hit.point + self.offset,
            normal: hit.normal,
            t: hit.t,
            u: hit.u,
            v: hit.v,
        })
    }

    fn bounding_box(&self, time: TimeRange) -> Box3 {
        self.shape.bounding_box(time).translate(self.offset)
    }

    fn sampler(&self, from: Vec3, time: f64) -> Option<Box<dyn Sampler>> {
        self.shape.sampler(from - self.offset, time)
    }

    fn is_empty(&self) -> bool {
        self.shape.is_empty()
    }
}

impl<S: Shape + Clone> Clone for Translate<S> {
    fn clone(&self) -> Self {
        Self {
            offset: self.offset,
            shape: self.shape.clone(),
        }
    }
}

impl<S: Shape> Translate<S> {
    pub fn new(offset: Vec3, shape: S) -> Self {
        Translate { offset, shape }
    }
}

#[derive(Debug)]
pub struct Rotate<S: Shape> {
    axis: Axis,
    theta: f64,
    shape: S,
}

impl<S: Shape> Shape for Rotate<S> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let ray = Ray::new(
            ray.origin.rotate_around(self.axis, -self.theta),
            ray.dir.rotate_around(self.axis, -self.theta),
            ray.time,
        );
        self.shape.hit(&ray, t_min, t_max).map(|hit| Hit {
            point: hit.point.rotate_around(self.axis, self.theta),
            normal: hit.normal.rotate_around(self.axis, self.theta),
            t: hit.t,
            u: hit.u,
            v: hit.v,
        })
    }

    fn bounding_box(&self, time: TimeRange) -> Box3 {
        self.shape
            .bounding_box(time)
            .iter_vertex()
            .map(|p| p.rotate_around(self.axis, self.theta))
            .map(|p| Box3::new(p, p))
            .fold(Box3::EMPTY, Box3::union)
    }

    fn sampler(&self, from: Vec3, time: f64) -> Option<Box<dyn Sampler>> {
        self.shape
            .sampler(from.rotate_around(self.axis, -self.theta), time)
            .map(|sampler| {
                Box::new(RotateSampler::new(self.axis, self.theta, sampler)) as Box<dyn Sampler>
            })
    }

    fn is_empty(&self) -> bool {
        self.shape.is_empty()
    }
}

impl<S: Shape + Clone> Clone for Rotate<S> {
    fn clone(&self) -> Self {
        Self {
            axis: self.axis,
            theta: self.theta,
            shape: self.shape.clone(),
        }
    }
}

impl<S: Shape> Rotate<S> {
    pub fn new(axis: Axis, theta: f64, shape: S) -> Self {
        Rotate { axis, theta, shape }
    }
}

#[derive(Debug)]
pub struct Union<S: Shape> {
    children: Vec<S>,
}

impl<S: Shape> Shape for Union<S> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        self.children
            .iter()
            .fold(None as Option<Hit>, |best, child| {
                let t_best = best.as_ref().map_or(t_max, |h| h.t);
                child.hit(ray, t_min, t_best).or(best)
            })
    }

    fn bounding_box(&self, time: TimeRange) -> Box3 {
        self.children
            .iter()
            .map(|child| child.bounding_box(time))
            .fold(Box3::EMPTY, Box3::union)
    }

    fn sampler(&self, from: Vec3, time: f64) -> Option<Box<dyn Sampler>> {
        let samplers = self
            .children
            .iter()
            .filter_map(|child| child.sampler(from, time))
            .collect_vec();
        if samplers.is_empty() {
            None
        } else {
            Some(Box::new(MixedSampler::new(samplers)))
        }
    }

    fn is_empty(&self) -> bool {
        self.children.is_empty()
    }
}

impl<S: Shape + Clone> Clone for Union<S> {
    fn clone(&self) -> Self {
        Self {
            children: self.children.clone(),
        }
    }
}

impl<S: Shape> Union<S> {
    pub fn new(children: impl IntoIterator<Item = S>) -> Self {
        Union {
            children: children
                .into_iter()
                .filter(|child| !child.is_empty())
                .collect(),
        }
    }
}

pub fn merge_shapes(shapes: impl IntoIterator<Item = Box<dyn Shape>>) -> Box<dyn Shape> {
    let mut shapes = shapes.into_iter().filter(|s| !s.is_empty()).collect_vec();
    match shapes.len() {
        0 => Box::new(EMPTY_SHAPE),
        1 => shapes.remove(0),
        _ => Box::new(Union::new(shapes)),
    }
}
