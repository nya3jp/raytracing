use crate::geom::Vec3;
use crate::material::Material;
use crate::ray::Ray;

pub struct Hit {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
}

pub trait Shape {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
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
        Some(Hit { point, normal, t })
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
    time0: f64,
    time1: f64,
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
        Some(Hit { point, normal, t })
    }
}

impl MovingSphere {
    pub fn new(center0: Vec3, center1: Vec3, time0: f64, time1: f64, radius: f64) -> Self {
        MovingSphere {
            center0,
            center1,
            time0,
            time1,
            radius,
        }
    }

    fn center_at(&self, time: f64) -> Vec3 {
        self.center0
            + (time - self.time0) * (self.time1 - self.time0) * (self.center1 - self.center0)
    }
}

pub struct HitMat<'a> {
    pub hit: Hit,
    pub material: &'a dyn Material,
}

pub trait Object {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitMat<'_>>;
}

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
}

impl<S: Shape, M: Material> PlainObject<S, M> {
    pub fn new(shape: S, material: M) -> Self {
        PlainObject { shape, material }
    }
}

pub struct Objects {
    objects: Vec<Box<dyn Object>>,
}

impl Object for Objects {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitMat<'_>> {
        let mut best: Option<HitMat<'_>> = None;
        for object in self.objects.iter() {
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
}

impl Objects {
    pub fn new(objects: Vec<Box<dyn Object>>) -> Self {
        Objects { objects }
    }
}
