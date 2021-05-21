use crate::geom::{Axis, Box3, Vec3};
use crate::material::Material;
use crate::ray::Ray;
use crate::time::TimeRange;
use std::f64::consts::PI;
use std::iter::FromIterator;

pub struct Hit {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
}

pub trait Shape {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
    fn bounding_box(&self, time: TimeRange) -> Box3;
}

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Shape for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let a = ray.dir.norm();
        let b2 = ray.dir.dot(oc);
        let c = oc.norm() - self.radius * self.radius;
        let discriminant = b2 * b2 - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let droot = discriminant.sqrt();
        let t = {
            let t_lo = (-b2 - droot) / a;
            if t_min <= t_lo && t_lo <= t_max {
                t_lo
            } else {
                let t_hi = (-b2 + droot) / a;
                if t_min <= t_hi && t_hi <= t_max {
                    t_hi
                } else {
                    return None;
                }
            }
        };

        let point = ray.at(t);
        let normal = (point - self.center) / self.radius;
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
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

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
        let a = ray.dir.norm();
        let b2 = ray.dir.dot(oc);
        let c = oc.norm() - self.radius * self.radius;
        let discriminant = b2 * b2 - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let droot = discriminant.sqrt();
        let t = {
            let t_lo = (-b2 - droot) / a;
            if t_min <= t_lo && t_lo <= t_max {
                t_lo
            } else {
                let t_hi = (-b2 + droot) / a;
                if t_min <= t_hi && t_hi <= t_max {
                    t_hi
                } else {
                    return None;
                }
            }
        };

        let point = ray.at(t);
        let normal = (point - center) / self.radius;
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

pub struct HitMat<'a> {
    pub hit: Hit,
    pub material: &'a dyn Material,
}

pub trait Object {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitMat<'_>>;
    fn bounding_box(&self, time: TimeRange) -> Box3;
}

pub type ObjectPtr = Box<dyn Object>;

pub struct PlainObject<S: Shape, M: Material> {
    shape: S,
    material: M,
}

impl<S: Shape, M: Material> Object for PlainObject<S, M> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitMat<'_>> {
        if let Some(hit) = self.shape.hit(ray, t_min, t_max) {
            Some(HitMat {
                hit,
                material: &self.material,
            })
        } else {
            None
        }
    }

    fn bounding_box(&self, time: TimeRange) -> Box3 {
        self.shape.bounding_box(time)
    }
}

impl<S: Shape, M: Material> PlainObject<S, M> {
    pub fn new(shape: S, material: M) -> Self {
        PlainObject { shape, material }
    }
}

impl<S: Shape + 'static, M: Material + 'static> PlainObject<S, M> {
    pub fn new_box(shape: S, material: M) -> ObjectPtr {
        Box::new(Self::new(shape, material))
    }
}

pub enum Objects {
    Leaf {
        objects: Vec<ObjectPtr>,
        bb: Box3,
    },
    Tree {
        left: ObjectPtr,
        right: ObjectPtr,
        bb: Box3,
    },
}

impl Object for Objects {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitMat<'_>> {
        match self {
            Objects::Leaf { objects, bb } => {
                if !ray.intersects(bb, t_min, t_max) {
                    return None;
                }
                let mut best: Option<HitMat<'_>> = None;
                for object in objects.iter() {
                    if let Some(hit) = object.hit(ray, t_min, t_max) {
                        if let Some(ref best_hit) = best {
                            if hit.hit.t < best_hit.hit.t {
                                best = Some(hit);
                            }
                        } else {
                            best = Some(hit);
                        }
                    }
                }
                best
            }
            Objects::Tree { left, right, bb } => {
                if !ray.intersects(bb, t_min, t_max) {
                    return None;
                }
                if let Some(left_hit) = left.hit(ray, t_min, t_max) {
                    Some(if let Some(right_hit) = right.hit(ray, t_min, t_max) {
                        if left_hit.hit.t < right_hit.hit.t {
                            left_hit
                        } else {
                            right_hit
                        }
                    } else {
                        left_hit
                    })
                } else {
                    right.hit(ray, t_min, t_max)
                }
            }
        }
    }

    fn bounding_box(&self, _time: TimeRange) -> Box3 {
        match self {
            Objects::Leaf { objects: _, bb } => *bb,
            Objects::Tree {
                left: _,
                right: _,
                bb,
            } => *bb,
        }
    }
}

impl Objects {
    pub fn new(objects: impl IntoIterator<Item = ObjectPtr>, time: TimeRange) -> Self {
        fn divide(mut objects: Vec<ObjectPtr>, axis: Axis, time: TimeRange) -> Objects {
            if objects.len() <= 5 {
                return Objects::new_leaf(objects, time);
            }
            objects.sort_by(|a, b| {
                a.bounding_box(time)
                    .min
                    .get(axis)
                    .partial_cmp(&b.bounding_box(time).min.get(axis))
                    .expect("NaN in coordinates")
            });
            let other = objects.split_off(objects.len() / 2);
            Objects::new_tree(
                Box::new(divide(objects, axis.next(), time)),
                Box::new(divide(other, axis.next(), time)),
                time,
            )
        }
        divide(Vec::from_iter(objects), Axis::X, time)
    }

    fn new_leaf(objects: Vec<ObjectPtr>, time: TimeRange) -> Self {
        let mut bb = Box3::EMPTY;
        for object in objects.iter() {
            bb = bb.union(object.bounding_box(time));
        }
        Objects::Leaf { objects, bb }
    }

    fn new_tree(left: ObjectPtr, right: ObjectPtr, time: TimeRange) -> Self {
        let bb = left.bounding_box(time).union(right.bounding_box(time));
        Objects::Tree { left, right, bb }
    }
}
