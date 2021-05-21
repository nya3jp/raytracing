use std::f64::consts::PI;

use crate::geom::{Box3, Vec3};
use crate::ray::Ray;
use crate::time::TimeRange;

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
