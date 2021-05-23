use std::iter::FromIterator;
use std::rc::Rc;

use crate::geom::{Axis, Box3, Vec3};
use crate::material::{Material, Scatter, VolumeMaterial};
use crate::ray::Ray;
use crate::rng::Rng;

use crate::shape::Shape;

use crate::time::TimeRange;
use rand::Rng as _;

#[derive(Clone, Debug)]
pub struct ObjectHit {
    pub t: f64,
    pub scatter: Scatter,
}

pub trait Object {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rng: &mut Rng) -> Option<ObjectHit>;
    fn bounding_box(&self, time: TimeRange) -> Box3;

    fn debug_object_tree(&self, time: TimeRange, depth: usize) {
        let bb = self.bounding_box(time);
        let size = bb.max - bb.min;
        eprintln!(
            "{}[{:.0}x{:.0}x{:.0}]",
            "  ".repeat(depth),
            size.x,
            size.y,
            size.z
        );
    }
}

pub struct TranslateObject<O: Object> {
    offset: Vec3,
    object: O,
}

impl<O: Object> Object for TranslateObject<O> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rng: &mut Rng) -> Option<ObjectHit> {
        let ray = Ray::new(ray.origin - self.offset, ray.dir, ray.time);
        if let Some(hit) = self.object.hit(&ray, t_min, t_max, rng) {
            Some(ObjectHit {
                t: hit.t,
                scatter: Scatter {
                    attenuation: hit.scatter.attenuation,
                    emit: hit.scatter.emit,
                    ray: hit
                        .scatter
                        .ray
                        .map(|r| Ray::new(r.origin + self.offset, r.dir, r.time)),
                },
            })
        } else {
            None
        }
    }

    fn bounding_box(&self, time: TimeRange) -> Box3 {
        self.object.bounding_box(time).translate(self.offset)
    }
}

impl<O: Object> TranslateObject<O> {
    pub fn new(offset: Vec3, object: O) -> Self {
        TranslateObject { offset, object }
    }
}

pub struct RotateObject<O: Object> {
    axis: Axis,
    theta: f64,
    object: O,
}

impl<O: Object> Object for RotateObject<O> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rng: &mut Rng) -> Option<ObjectHit> {
        let ray = Ray::new(
            ray.origin.rotate_around(self.axis, -self.theta),
            ray.dir.rotate_around(self.axis, -self.theta),
            ray.time,
        );
        if let Some(hit) = self.object.hit(&ray, t_min, t_max, rng) {
            Some(ObjectHit {
                t: hit.t,
                scatter: Scatter {
                    attenuation: hit.scatter.attenuation,
                    emit: hit.scatter.emit,
                    ray: hit.scatter.ray.map(|r| {
                        Ray::new(
                            r.origin.rotate_around(self.axis, self.theta),
                            r.dir.rotate_around(self.axis, self.theta),
                            r.time,
                        )
                    }),
                },
            })
        } else {
            None
        }
    }

    fn bounding_box(&self, time: TimeRange) -> Box3 {
        self.object
            .bounding_box(time)
            .iter_vertex()
            .map(|p| p.rotate_around(self.axis, self.theta))
            .map(|p| Box3::new(p, p))
            .fold(Box3::EMPTY, Box3::union)
    }
}

impl<O: Object> RotateObject<O> {
    pub fn new(axis: Axis, theta: f64, object: O) -> Self {
        RotateObject {
            axis,
            theta,
            object,
        }
    }
}

pub type ObjectPtr = Rc<dyn Object>;

pub struct SolidObject<S: Shape, M: Material> {
    shape: S,
    material: M,
}

impl<S: Shape, M: Material> Object for SolidObject<S, M> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rng: &mut Rng) -> Option<ObjectHit> {
        if let Some(hit) = self.shape.hit(ray, t_min, t_max) {
            let scatter = self.material.scatter(ray, &hit, rng);
            Some(ObjectHit { t: hit.t, scatter })
        } else {
            None
        }
    }

    fn bounding_box(&self, time: TimeRange) -> Box3 {
        self.shape.bounding_box(time)
    }
}

impl<S: Shape, M: Material> SolidObject<S, M> {
    pub fn new(shape: S, material: M) -> Self {
        SolidObject { shape, material }
    }
}

impl<S: Shape + 'static, M: Material + 'static> SolidObject<S, M> {
    pub fn new_rc(shape: S, material: M) -> ObjectPtr {
        Rc::new(Self::new(shape, material))
    }
}

pub struct VolumeObject<S: Shape, V: VolumeMaterial> {
    boundary: S,
    volume: V,
    density: f64,
}

impl<S: Shape, V: VolumeMaterial> Object for VolumeObject<S, V> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rng: &mut Rng) -> Option<ObjectHit> {
        let hit0 = self.boundary.hit(ray, f64::NEG_INFINITY, f64::INFINITY)?;
        let hit1 = self.boundary.hit(ray, hit0.t + 1e-8, f64::INFINITY)?;
        let t0 = hit0.t.max(t_min);
        let t1 = hit1.t.min(t_max);
        if t0 >= t1 {
            return None;
        }
        let inside_distance = (t1 - t0) * ray.dir.abs();
        let hit_distance = -rng.gen::<f64>().ln() / self.density;
        if hit_distance > inside_distance {
            return None;
        }
        let t = hit_distance / ray.dir.abs() + t0;
        let point = ray.at(t);
        Some(ObjectHit {
            t,
            scatter: self.volume.scatter(ray, point, rng),
        })
    }

    fn bounding_box(&self, time: TimeRange) -> Box3 {
        self.boundary.bounding_box(time)
    }
}

impl<S: Shape, V: VolumeMaterial> VolumeObject<S, V> {
    pub fn new(boundary: S, volume: V, density: f64) -> Self {
        VolumeObject {
            boundary,
            volume,
            density,
        }
    }
}

impl<S: Shape + 'static, V: VolumeMaterial + 'static> VolumeObject<S, V> {
    pub fn new_rc(shape: S, volume: V, density: f64) -> ObjectPtr {
        Rc::new(Self::new(shape, volume, density))
    }
}

#[derive(Clone)]
pub struct Objects {
    children: Vec<ObjectPtr>,
    bb: Box3,
}

impl Object for Objects {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rng: &mut Rng) -> Option<ObjectHit> {
        if !ray.intersects(&self.bb, t_min, t_max) {
            return None;
        }
        self.children
            .iter()
            .map(|obj| obj.hit(ray, t_min, t_max, rng))
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

    fn bounding_box(&self, _time: TimeRange) -> Box3 {
        self.bb
    }

    fn debug_object_tree(&self, time: TimeRange, depth: usize) {
        let size = self.bb.max - self.bb.min;
        eprintln!(
            "{}[{:.0}x{:.0}x{:.0}] ({} children)",
            "  ".repeat(depth),
            size.x,
            size.y,
            size.z,
            self.children.len()
        );
        for child in self.children.iter() {
            child.debug_object_tree(time, depth + 1);
        }
    }
}

impl Objects {
    pub fn new(objects: impl IntoIterator<Item = ObjectPtr>, time: TimeRange) -> Self {
        fn divide(mut objects: Vec<ObjectPtr>, axis: Axis, time: TimeRange) -> Objects {
            if objects.len() <= 5 {
                return Objects::new_flat(objects, time);
            }
            objects.sort_by(|a, b| {
                a.bounding_box(time)
                    .min
                    .get(axis)
                    .partial_cmp(&b.bounding_box(time).min.get(axis))
                    .expect("NaN in coordinates")
            });
            let other = objects.split_off(objects.len() / 2);
            Objects::new_flat(
                vec![
                    Rc::new(divide(objects, axis.next(), time)),
                    Rc::new(divide(other, axis.next(), time)),
                ],
                time,
            )
        }
        divide(Vec::from_iter(objects), Axis::X, time)
    }

    pub fn new_flat(objects: Vec<ObjectPtr>, time: TimeRange) -> Self {
        let mut bb = Box3::EMPTY;
        for object in objects.iter() {
            bb = bb.union(object.bounding_box(time));
        }
        Objects {
            children: objects,
            bb,
        }
    }
}
