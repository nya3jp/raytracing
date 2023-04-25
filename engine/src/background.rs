use crate::color::Color;
use crate::ray::Ray;

#[derive(Clone, Copy, Debug)]
pub enum Background {
    Sky,
    Black,
}

impl Background {
    pub fn color(self, ray: &Ray) -> Color {
        match self {
            Background::Sky => {
                let t = 0.5 * (ray.dir.y + 1.0);
                (1.0 - t) * Color::WHITE + t * Color::new(0.5, 0.7, 1.0)
            }
            Background::Black => Color::BLACK,
        }
    }
}
