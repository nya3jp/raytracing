use crate::color::Color;
use crate::geom::Vec3;
use crate::ray::Ray;
use crate::rng::Rng;
use crate::shape::Hit;
use crate::texture::Texture;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Scatter {
    pub attenuation: Color,
    pub emit: Color,
    pub ray: Option<Ray>,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &Hit, rng: &mut Rng) -> Scatter;
}

pub trait VolumeMaterial {
    fn scatter(&self, ray: &Ray, point: Vec3, rng: &mut Rng) -> Scatter;
}

#[derive(Clone)]
pub struct Lambertian {
    texture: Rc<dyn Texture>,
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &Hit, rng: &mut Rng) -> Scatter {
        let out_normal = if ray.dir.dot(hit.normal) < 0.0 {
            hit.normal
        } else {
            -hit.normal
        };
        Scatter {
            attenuation: self.texture.color(hit.u, hit.v, hit.point),
            emit: Color::BLACK,
            ray: Some(Ray::new(
                hit.point,
                (out_normal + Vec3::random_in_unit_sphere(rng)).unit(),
                ray.time,
            )),
        }
    }
}

impl Lambertian {
    pub fn new(texture: Rc<dyn Texture>) -> Lambertian {
        Lambertian { texture }
    }
}

#[derive(Clone)]
pub struct Metal {
    texture: Rc<dyn Texture>,
    fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit, rng: &mut Rng) -> Scatter {
        Scatter {
            attenuation: self.texture.color(hit.u, hit.v, hit.point),
            emit: Color::BLACK,
            ray: Some(Ray::new(
                hit.point,
                reflect(ray.dir, hit.normal) + Vec3::random_in_unit_sphere(rng) * self.fuzz,
                ray.time,
            )),
        }
    }
}

impl Metal {
    pub fn new(texture: Rc<dyn Texture>, fuzz: f64) -> Metal {
        Metal { texture, fuzz }
    }
}

#[derive(Clone)]
pub struct Dielectric {
    texture: Rc<dyn Texture>,
    index: f64,
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &Hit, _rng: &mut Rng) -> Scatter {
        Scatter {
            attenuation: self.texture.color(hit.u, hit.v, hit.point),
            emit: Color::BLACK,
            ray: Some(Ray::new(
                hit.point,
                {
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
                },
                ray.time,
            )),
        }
    }
}

impl Dielectric {
    pub fn new(texture: Rc<dyn Texture>, index: f64) -> Dielectric {
        Dielectric { texture, index }
    }
}

#[derive(Clone)]
pub struct DiffuseLight {
    texture: Rc<dyn Texture>,
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray: &Ray, hit: &Hit, _rng: &mut Rng) -> Scatter {
        Scatter {
            attenuation: Color::BLACK,
            emit: self.texture.color(hit.u, hit.v, hit.point),
            ray: None,
        }
    }
}

impl DiffuseLight {
    pub fn new(texture: Rc<dyn Texture>) -> DiffuseLight {
        DiffuseLight { texture }
    }
}

#[derive(Clone)]
pub struct Fog {
    color: Color,
}

impl VolumeMaterial for Fog {
    fn scatter(&self, ray: &Ray, point: Vec3, rng: &mut Rng) -> Scatter {
        Scatter {
            attenuation: self.color,
            emit: Color::BLACK,
            ray: Some(Ray::new(point, Vec3::random_in_unit_sphere(rng), ray.time)),
        }
    }
}

impl Fog {
    pub fn new(color: Color) -> Fog {
        Fog { color }
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
