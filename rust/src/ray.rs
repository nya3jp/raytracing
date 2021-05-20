use crate::geom::{Axis, Box3, Vec3};

#[derive(Clone, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3, time: f64) -> Self {
        Ray { origin, dir, time }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.dir * t
    }

    pub fn intersects(&self, bb: &Box3, t_min: f64, t_max: f64) -> bool {
        // FIXME: Handle NaN.
        fn range(ray: &Ray, bb: &Box3, axis: Axis) -> (f64, f64) {
            let t0 = (bb.min.get(axis) - ray.origin.get(axis)) / ray.dir.get(axis);
            let t1 = (bb.max.get(axis) - ray.origin.get(axis)) / ray.dir.get(axis);
            if t0 < t1 {
                (t0, t1)
            } else {
                (t1, t0)
            }
        }
        let (x_min, x_max) = range(self, bb, Axis::X);
        let (y_min, y_max) = range(self, bb, Axis::Y);
        let (z_min, z_max) = range(self, bb, Axis::Z);
        t_min.max(x_min).max(y_min).max(z_min) <= t_max.min(x_max).min(y_max).min(z_max)
    }
}
