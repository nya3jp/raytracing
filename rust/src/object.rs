use std::iter::FromIterator;
use std::rc::Rc;

use crate::geom::{Axis, Box3};
use crate::material::Material;
use crate::ray::Ray;
use crate::shape::{Hit, Shape};
use crate::time::TimeRange;

pub struct HitMat<'a> {
    pub hit: Hit,
    pub material: &'a dyn Material,
}

pub trait Object {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitMat<'_>>;
    fn bounding_box(&self, time: TimeRange) -> Box3;
}

pub type ObjectPtr = Rc<dyn Object>;

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
    pub fn new_rc(shape: S, material: M) -> ObjectPtr {
        Rc::new(Self::new(shape, material))
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
                Rc::new(divide(objects, axis.next(), time)),
                Rc::new(divide(other, axis.next(), time)),
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
