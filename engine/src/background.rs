use crate::color::Color;
use crate::ray::Ray;

#[derive(Clone, Copy, Debug)]
pub enum Background {
    SKY,
    BLACK,
}

impl Background {
    pub fn color(self, ray: &Ray) -> Color {
        match self {
            Background::SKY => {
                let t = 0.5 * (ray.dir.y + 1.0);
                (1.0 - t) * Color::WHITE + t * Color::new(0.5, 0.7, 1.0)
            }
            Background::BLACK => Color::BLACK,
        }
    }
}
