use crate::geom::Vec3;

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
}
