use std::f64::consts::PI;

use crate::geom::{Axis, Box3, Vec3};
use crate::ray::Ray;
use crate::time::TimeRange;

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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
        let normal = Vec3::new(1.0, 0.0, 0.0).rotate_axes(self.axis);
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
            Vec3::new(self.a, self.b_min, self.c_min).rotate_axes(self.axis),
            Vec3::new(self.a, self.b_max, self.c_max).rotate_axes(self.axis),
        )
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
pub struct Box {
    faces: [Rectangle; 6],
}

impl Shape for Box {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        self.faces
            .iter()
            .map(|r| r.hit(ray, t_min, t_max))
            .fold(None, |a, b| {
                if let Some(ref hit_a) = a {
                    if let Some(ref hit_b) = b {
                        if hit_a.t < hit_b.t {
                            a
                        } else {
                            b
                        }
                    } else {
                        a
                    }
                } else {
                    b
                }
            })
    }

    fn bounding_box(&self, time: TimeRange) -> Box3 {
        self.faces
            .iter()
            .map(|r| r.bounding_box(time))
            .fold(Box3::EMPTY, Box3::union)
    }
}

impl Box {
    pub fn new(b: Box3) -> Box {
        Box {
            faces: [
                Rectangle::new(Axis::X, b.min.x, b.min.y, b.max.y, b.min.z, b.max.z),
                Rectangle::new(Axis::X, b.max.x, b.min.y, b.max.y, b.min.z, b.max.z),
                Rectangle::new(Axis::Y, b.min.y, b.min.z, b.max.z, b.min.x, b.max.x),
                Rectangle::new(Axis::Y, b.max.y, b.min.z, b.max.z, b.min.x, b.max.x),
                Rectangle::new(Axis::Z, b.min.z, b.min.x, b.max.x, b.min.y, b.max.y),
                Rectangle::new(Axis::Z, b.max.z, b.min.x, b.max.x, b.min.y, b.max.y),
            ],
        }
    }
}
