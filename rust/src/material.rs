use crate::color::Color;
use crate::geom::Vec3;
use crate::object::Hit;
use crate::ray::Ray;
use crate::rng::Rng;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &Hit, rng: &mut Rng) -> (Color, Option<Ray>);
}

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
pub struct Dielectric {
    index: f64,
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &Hit, _rng: &mut Rng) -> (Color, Option<Ray>) {
        (
            Color::WHITE,
            Some(Ray::new(hit.point, {
                let ratio = if ray.dir.dot(hit.normal) > 0.0 {
                    self.index
                } else {
                    1.0 / self.index
                };
                if let Some(new_dir) = refract(ray.dir, hit.normal, ratio) {
                    new_dir
                } else {
                    reflect(ray.dir, hit.normal)
                }
            })),
        )
    }
}

impl Dielectric {
    pub fn new(index: f64) -> Dielectric {
        Dielectric { index }
    }
}

fn reflect(in_dir: Vec3, normal: Vec3) -> Vec3 {
    in_dir - (normal.dot(in_dir) * 2.0) * normal
}

fn refract(in_dir: Vec3, normal: Vec3, ratio: f64) -> Option<Vec3> {
    let in_dir_unit = in_dir.unit();
    let in_normal = if in_dir_unit.dot(normal) < 0.0 {
        normal
    } else {
        -normal
    };
    let cos = -in_dir_unit.dot(in_normal).min(1.0);
    let sin = (1.0 - cos * cos).sqrt();
    if ratio * sin > 1.0 {
        return None;
    }
    let out_dir_perp = (in_dir_unit + in_normal * cos) * ratio;
    let out_dir_para = -(1.0 - out_dir_perp.norm()).abs().sqrt() * in_normal;
    Some(out_dir_perp + out_dir_para)
}
