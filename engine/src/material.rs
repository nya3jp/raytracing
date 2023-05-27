use crate::color::Color;
use crate::geom::{dot, Vec3};
use crate::physics::{reflect, reflectance, refract};
use crate::ray::Ray;
use crate::rng::Rng;
use crate::sampler::{ConstantSampler, LambertianSampler, Sampler, SphereSampler};
use crate::shape::Hit;
use crate::texture::Texture;
use rand::Rng as _;

#[derive(Debug)]
pub struct Scatter {
    pub point: Vec3,
    pub albedo: Color,
    pub emit: Color,
    pub sampler: Option<Box<dyn Sampler>>,
}

pub trait Material: Sync + Send {
    fn scatter(&self, ray: &Ray, hit: &Hit, rng: &mut Rng) -> Scatter;
    fn important(&self) -> bool;
}

pub trait VolumeMaterial: Sync + Send {
    fn scatter(&self, ray: &Ray, point: Vec3, rng: &mut Rng) -> Scatter;
}

#[derive(Clone)]
pub struct Lambertian<T: Texture> {
    texture: T,
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, ray: &Ray, hit: &Hit, _rng: &mut Rng) -> Scatter {
        let out_normal = if dot(ray.dir, hit.normal) < 0.0 {
            hit.normal
        } else {
            -hit.normal
        };
        Scatter {
            point: hit.point,
            albedo: self.texture.color(hit.u, hit.v, hit.point),
            emit: Color::BLACK,
            sampler: Some(Box::new(LambertianSampler::new(out_normal))),
            // sampler: Some(Box::new(SphereSampler::new(out_normal.into_vec3(), 1.0))),
        }
    }

    fn important(&self) -> bool {
        false
    }
}

impl<T: Texture> Lambertian<T> {
    pub fn new(texture: T) -> Self {
        Lambertian { texture }
    }
}

#[derive(Clone)]
pub struct Metal<T: Texture> {
    texture: T,
    fuzz: f64,
}

impl<T: Texture> Material for Metal<T> {
    fn scatter(&self, ray: &Ray, hit: &Hit, _rng: &mut Rng) -> Scatter {
        Scatter {
            point: hit.point,
            albedo: self.texture.color(hit.u, hit.v, hit.point),
            emit: Color::BLACK,
            sampler: Some(Box::new(SphereSampler::new(
                reflect(ray.dir, hit.normal).into_vec3(),
                self.fuzz,
            ))),
        }
    }

    fn important(&self) -> bool {
        true
    }
}

impl<T: Texture> Metal<T> {
    pub fn new(texture: T, fuzz: f64) -> Self {
        Metal { texture, fuzz }
    }
}

#[derive(Clone)]
pub struct Dielectric {
    index: f64,
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &Hit, rng: &mut Rng) -> Scatter {
        let new_dir = {
            let ratio = if dot(ray.dir, hit.normal) > 0.0 {
                self.index
            } else {
                1.0 / self.index
            };
            if rng.gen::<f64>() < reflectance(ray.dir, hit.normal, ratio) {
                reflect(ray.dir, hit.normal)
            } else if let Some(new_dir) = refract(ray.dir, hit.normal, ratio) {
                new_dir
            } else {
                reflect(ray.dir, hit.normal)
            }
        };
        Scatter {
            point: hit.point,
            albedo: Color::WHITE,
            emit: Color::BLACK,
            sampler: Some(Box::new(ConstantSampler::new(new_dir))),
        }
    }

    fn important(&self) -> bool {
        true
    }
}

impl Dielectric {
    pub fn new(index: f64) -> Self {
        Dielectric { index }
    }
}

#[derive(Clone)]
pub struct DiffuseLight<T: Texture> {
    texture: T,
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(&self, _ray: &Ray, hit: &Hit, _rng: &mut Rng) -> Scatter {
        Scatter {
            point: hit.point,
            albedo: Color::BLACK,
            emit: self.texture.color(hit.u, hit.v, hit.point),
            sampler: None,
        }
    }

    fn important(&self) -> bool {
        true
    }
}

impl<T: Texture> DiffuseLight<T> {
    pub fn new(texture: T) -> Self {
        DiffuseLight { texture }
    }
}

#[derive(Clone)]
pub struct Fog {
    color: Color,
}

impl VolumeMaterial for Fog {
    fn scatter(&self, _ray: &Ray, point: Vec3, _rng: &mut Rng) -> Scatter {
        Scatter {
            point,
            albedo: self.color,
            emit: Color::BLACK,
            sampler: Some(Box::new(SphereSampler::new(Vec3::ZERO, 1.0))),
        }
    }
}

impl Fog {
    pub fn new(color: Color) -> Self {
        Fog { color }
    }
}
