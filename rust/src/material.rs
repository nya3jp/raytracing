use crate::color::Color;
use crate::geom::{IntoVec3, Vec3, Vec3Unit};
use crate::ray::Ray;
use crate::rng::Rng;
use crate::sampler::{ConstantSampler, LambartianSampler, Sampler, SphereSampler};
use crate::shape::Hit;
use crate::texture::Texture;
use rand::Rng as _;

#[derive(Debug)]
pub struct Scatter {
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
        let out_normal = if ray.dir.dot(hit.normal) < 0.0 {
            hit.normal
        } else {
            -hit.normal
        };
        Scatter {
            albedo: self.texture.color(hit.u, hit.v, hit.point),
            emit: Color::BLACK,
            sampler: Some(Box::new(LambartianSampler::new(out_normal))),
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
pub struct Dielectric<T: Texture> {
    texture: T,
    index: f64,
}

impl<T: Texture> Material for Dielectric<T> {
    fn scatter(&self, ray: &Ray, hit: &Hit, rng: &mut Rng) -> Scatter {
        let new_dir = {
            let ratio = if ray.dir.dot(hit.normal) > 0.0 {
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
            albedo: self.texture.color(hit.u, hit.v, hit.point),
            emit: Color::BLACK,
            sampler: Some(Box::new(ConstantSampler::new(new_dir))),
        }
    }

    fn important(&self) -> bool {
        true
    }
}

impl<T: Texture> Dielectric<T> {
    pub fn new(texture: T, index: f64) -> Self {
        Dielectric { texture, index }
    }
}

#[derive(Clone)]
pub struct DiffuseLight<T: Texture> {
    texture: T,
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(&self, _ray: &Ray, hit: &Hit, _rng: &mut Rng) -> Scatter {
        Scatter {
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
    fn scatter(&self, _ray: &Ray, _point: Vec3, _rng: &mut Rng) -> Scatter {
        Scatter {
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

fn reflectance(in_dir: Vec3Unit, normal: Vec3Unit, ratio: f64) -> f64 {
    let in_normal = if in_dir.dot(normal) < 0.0 {
        normal
    } else {
        -normal
    };
    let cos = -in_dir.dot(in_normal).min(1.0);
    let r0 = ((1.0 - ratio) / (1.0 + ratio)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}

fn reflect(in_dir: Vec3Unit, normal: Vec3Unit) -> Vec3Unit {
    (in_dir - (normal.dot(in_dir) * 2.0) * normal).unit()
}

fn refract(in_dir: Vec3Unit, normal: Vec3Unit, ratio: f64) -> Option<Vec3Unit> {
    let in_normal = if in_dir.dot(normal) < 0.0 {
        normal
    } else {
        -normal
    };
    let cos = -in_dir.dot(in_normal).min(1.0);
    let sin = (1.0 - cos * cos).sqrt();
    if ratio * sin > 1.0 {
        return None;
    }
    let out_dir_perp = (in_dir + in_normal * cos) * ratio;
    let out_dir_para = -(1.0 - out_dir_perp.norm()).abs().sqrt() * in_normal;
    Some((out_dir_perp + out_dir_para).unit())
}
