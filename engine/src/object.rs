use crate::color::Color;
use crate::geom::{Axis, Box3, Vec3, dot, cross};
use crate::material::{Material, Scatter, VolumeMaterial};
use crate::ray::Ray;
use crate::rng::Rng;
use crate::sampler::{ConstantSampler, RotateSampler, Sampler};
use crate::shape::{merge_shapes, PortalShape, Rotate, Shape, Translate, EMPTY_SHAPE};
use crate::time::TimeRange;
use rand::Rng as _;
use std::iter::FromIterator;
use std::sync::Arc;

#[derive(Debug)]
pub struct ObjectHit {
    pub t: f64,
    pub scatter: Scatter,
}

pub trait Object: Sync + Send {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rng: &mut Rng) -> Option<ObjectHit>;
    fn bounding_box(&self, time: TimeRange) -> Box3;
    fn important_shape(&self) -> Box<dyn Shape>;
}

pub struct TranslateObject<O: Object> {
    offset: Vec3,
    object: O,
}

impl<O: Object> Object for TranslateObject<O> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rng: &mut Rng) -> Option<ObjectHit> {
        let ray = Ray::new(ray.origin - self.offset, ray.dir, ray.time);
        self.object
            .hit(&ray, t_min, t_max, rng)
            .map(|hit| ObjectHit {
                t: hit.t,
                scatter: Scatter {
                    point: hit.scatter.point + self.offset,
                    emit: hit.scatter.emit,
                    albedo: hit.scatter.albedo,
                    sampler: hit.scatter.sampler,
                },
            })
    }

    fn bounding_box(&self, time: TimeRange) -> Box3 {
        self.object.bounding_box(time).translate(self.offset)
    }

    fn important_shape(&self) -> Box<dyn Shape> {
        Box::new(Translate::new(self.offset, self.object.important_shape()))
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
        self.object
            .hit(&ray, t_min, t_max, rng)
            .map(|hit| ObjectHit {
                t: hit.t,
                scatter: Scatter {
                    point: hit.scatter.point.rotate_around(self.axis, self.theta),
                    albedo: hit.scatter.albedo,
                    emit: hit.scatter.emit,
                    sampler: hit.scatter.sampler.map(|s| {
                        Box::new(RotateSampler::new(self.axis, self.theta, s)) as Box<dyn Sampler>
                    }),
                },
            })
    }

    fn bounding_box(&self, time: TimeRange) -> Box3 {
        self.object
            .bounding_box(time)
            .iter_vertex()
            .map(|p| p.rotate_around(self.axis, self.theta))
            .map(|p| Box3::new(p, p))
            .fold(Box3::EMPTY, Box3::union)
    }

    fn important_shape(&self) -> Box<dyn Shape> {
        Box::new(Rotate::new(
            self.axis,
            self.theta,
            self.object.important_shape(),
        ))
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

pub type ObjectPtr = Arc<dyn Object>;

pub struct SolidObject<S: Shape, M: Material> {
    shape: S,
    material: M,
}

impl<S: Shape + Clone + 'static, M: Material> Object for SolidObject<S, M> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rng: &mut Rng) -> Option<ObjectHit> {
        self.shape.hit(ray, t_min, t_max).map(|hit| ObjectHit {
            t: hit.t,
            scatter: self.material.scatter(ray, &hit, rng),
        })
    }

    fn bounding_box(&self, time: TimeRange) -> Box3 {
        self.shape.bounding_box(time)
    }

    fn important_shape(&self) -> Box<dyn Shape> {
        if self.material.important() {
            Box::new(self.shape.clone())
        } else {
            Box::new(EMPTY_SHAPE)
        }
    }
}

impl<S: Shape, M: Material> SolidObject<S, M> {
    pub fn new(shape: S, material: M) -> Self {
        SolidObject { shape, material }
    }
}

impl<S: Shape + Clone + 'static, M: Material + 'static> SolidObject<S, M> {
    pub fn new_rc(shape: S, material: M) -> ObjectPtr {
        Arc::new(Self::new(shape, material))
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
        let inside_distance = t1 - t0;
        let hit_distance = -rng.gen::<f64>().ln() / self.density;
        if hit_distance > inside_distance {
            return None;
        }
        let t = hit_distance + t0;
        let point = ray.at(t);
        Some(ObjectHit {
            t,
            scatter: self.volume.scatter(ray, point, rng),
        })
    }

    fn bounding_box(&self, time: TimeRange) -> Box3 {
        self.boundary.bounding_box(time)
    }

    fn important_shape(&self) -> Box<dyn Shape> {
        Box::new(EMPTY_SHAPE)
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
        Arc::new(Self::new(shape, volume, density))
    }
}

pub struct PortalObject<S: PortalShape, T: PortalShape> {
    source: S,
    target: T,
}

impl<S: PortalShape, T: PortalShape> Object for PortalObject<S, T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, _rng: &mut Rng) -> Option<ObjectHit> {
        self.target.hit(ray, t_min, t_max).map(|hit| {
            let target = self.target.surface(hit.u, hit.v);
            let source = self.source.surface(hit.u, hit.v);
            let new_dir = (dot(ray.dir, target.du) * source.du
                + dot(ray.dir, target.dv) * source.dv
                + dot(ray.dir, cross(target.du, target.dv)) * cross(source.du, source.dv))
            .unit();
            /*
            eprintln!("====================");
            eprintln!("target = {:?}", target);
            eprintln!("source = {:?}", source);
            eprintln!("in_dir = {:?}", ray.dir);
            eprintln!("out_dir = {:?}", new_dir);
            panic!("stop");
            */
            ObjectHit {
                t: hit.t,
                scatter: Scatter {
                    point: source.point,
                    emit: Color::BLACK,
                    albedo: Color::WHITE,
                    sampler: Some(Box::new(ConstantSampler::new(new_dir))),
                },
            }
        })
    }

    fn bounding_box(&self, time: TimeRange) -> Box3 {
        self.target.bounding_box(time)
    }

    fn important_shape(&self) -> Box<dyn Shape> {
        Box::new(EMPTY_SHAPE)
    }
}

impl<S: PortalShape, T: PortalShape> PortalObject<S, T> {
    pub fn new(source: S, target: T) -> Self {
        PortalObject { source, target }
    }
}

impl<S: PortalShape + 'static, T: PortalShape + 'static> PortalObject<S, T> {
    pub fn new_rc(source: S, target: T) -> ObjectPtr {
        Arc::new(Self::new(source, target))
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
            .fold(None as Option<ObjectHit>, |best, obj| {
                let t_best = best.as_ref().map_or(t_max, |h| h.t);
                obj.hit(ray, t_min, t_best, rng).or(best)
            })
    }

    fn bounding_box(&self, _time: TimeRange) -> Box3 {
        self.bb
    }

    fn important_shape(&self) -> Box<dyn Shape> {
        merge_shapes(self.children.iter().map(|child| child.important_shape()))
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
                    Arc::new(divide(objects, axis.next(), time)),
                    Arc::new(divide(other, axis.next(), time)),
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

pub struct GlobalVolume<V: VolumeMaterial, O: Object> {
    volume: V,
    radius2: f64,
    neg_inv_density: f64,
    object: O,
}

impl<V: VolumeMaterial, O: Object> Object for GlobalVolume<V, O> {
    fn hit(&self, ray: &Ray, t_min: f64, _t_max: f64, rng: &mut Rng) -> Option<ObjectHit> {
        let t = rng.gen::<f64>().ln() * self.neg_inv_density;
        let hit = self.object.hit(ray, t_min, t, rng);
        if hit.is_some() {
            hit
        } else {
            let point = ray.at(t);
            if point.norm() > self.radius2 {
                None
            } else {
                Some(ObjectHit {
                    t,
                    scatter: self.volume.scatter(ray, point, rng),
                })
            }
        }
    }

    fn bounding_box(&self, time: TimeRange) -> Box3 {
        self.object.bounding_box(time)
    }

    fn important_shape(&self) -> Box<dyn Shape> {
        self.object.important_shape()
    }
}

impl<V: VolumeMaterial, O: Object> GlobalVolume<V, O> {
    pub fn new(volume: V, radius: f64, density: f64, object: O) -> Self {
        GlobalVolume {
            volume,
            radius2: radius * radius,
            neg_inv_density: -1.0 / density,
            object,
        }
    }
}
