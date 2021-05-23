use crate::color::Color;
use crate::ray::Ray;

pub trait Background {
    fn color(&self, ray: &Ray) -> Color;
}

pub struct Sky();

impl Background for Sky {
    fn color(&self, ray: &Ray) -> Color {
        let t = 0.5 * (ray.dir.y + 1.0);
        (1.0 - t) * Color::WHITE + t * Color::new(0.5, 0.7, 1.0)
    }
}

impl Sky {
    pub fn new() -> Self {
        Sky {}
    }
}

pub struct Black();

impl Background for Black {
    fn color(&self, _ray: &Ray) -> Color {
        Color::BLACK
    }
}

impl Black {
    pub fn new() -> Self {
        Black {}
    }
}
