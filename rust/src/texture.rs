use crate::color::Color;
use crate::geom::Vec3;

pub trait Texture {
    fn color(&self, u: f64, v: f64, p: Vec3) -> Color;
}

#[derive(Clone, Copy, Debug)]
pub struct SolidColor(Color);

impl Texture for SolidColor {
    fn color(&self, u: f64, v: f64, p: Vec3) -> Color {
        self.0
    }
}

impl SolidColor {
    pub fn new(color: Color) -> Self {
        SolidColor(color)
    }
}

pub struct Checker {
    even: Box<dyn Texture>,
    odd: Box<dyn Texture>,
    stride: f64,
}

impl Texture for Checker {
    fn color(&self, u: f64, v: f64, p: Vec3) -> Color {
        let alt = |w: f64| (w / self.stride).rem_euclid(2.0) as isize;
        let bit = alt(p.x) ^ alt(p.y) ^ alt(p.z);
        if bit & 1 == 0 {
            self.even.color(u, v, p)
        } else {
            self.odd.color(u, v, p)
        }
    }
}

impl Checker {
    pub fn new(even: Box<dyn Texture>, odd: Box<dyn Texture>, stride: f64) -> Self {
        Checker { even, odd, stride }
    }
}
