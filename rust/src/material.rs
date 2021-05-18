use crate::color::Color;
use crate::geom::{Ray, Vec3};
use crate::object::Hit;
use crate::rng::Rng;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &Hit, rng: &mut Rng) -> (Color, Option<Ray>);
}

pub struct Lambertian {
    color: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &Hit, rng: &mut Rng) -> (Color, Option<Ray>) {
        (
            self.color,
            Some(Ray::new(
                hit.point,
                (hit.normal + Vec3::random_in_unit_sphere(rng)).unit(),
            )),
        )
    }
}

impl Lambertian {
    pub fn new(color: Color) -> Lambertian {
        Lambertian { color }
    }
}

pub struct Metal {
    color: Color,
    fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit, rng: &mut Rng) -> (Color, Option<Ray>) {
        (
            self.color,
            Some(Ray::new(
                hit.point,
                reflect(ray.dir, hit.normal) + Vec3::random_in_unit_sphere(rng) * self.fuzz,
            )),
        )
    }
}

impl Metal {
    pub fn new(color: Color, fuzz: f64) -> Metal {
        Metal { color, fuzz }
    }
}

fn reflect(in_dir: Vec3, normal: Vec3) -> Vec3 {
    in_dir - (normal.dot(in_dir) * 2.0) * normal
}
