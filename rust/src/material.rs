use crate::color::Color;
use crate::geom::Ray;
use crate::object::Hit;
use crate::rng::Rng;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &Hit, rng: &mut Rng) -> (Color, Option<Ray>);
}

pub struct Lambertian {
    color: Color,
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &Hit, rng: &mut Rng) -> (Color, Option<Ray>) {
        todo!();
    }
}

impl Lambertian {
    pub fn new(color: Color) -> Lambertian {
        Lambertian { color }
    }
}
