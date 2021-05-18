use crate::color::Color;
use crate::geom::Vec3;
use crate::material::{Lambertian, Metal};
use crate::object::{Objects, PlainObject, Sphere};
use crate::renderer::Camera;

pub fn sample0(aspect_ratio: f64) -> (Camera, Objects) {
    let camera = Camera::new(aspect_ratio);
    let objects = Objects::new(vec![
        Box::new(PlainObject::new(
            Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5),
            Lambertian::new(Color::new(0.5, 0.5, 0.5)),
        )),
        Box::new(PlainObject::new(
            Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0),
            Lambertian::new(Color::new(0.5, 0.5, 0.5)),
        )),
    ]);
    (camera, objects)
}

pub fn sample9(aspect_ratio: f64) -> (Camera, Objects) {
    let camera = Camera::new(aspect_ratio);
    let objects = Objects::new(vec![
        Box::new(PlainObject::new(
            Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0),
            Lambertian::new(Color::new(0.8, 0.8, 0.0)),
        )),
        Box::new(PlainObject::new(
            Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5),
            Lambertian::new(Color::new(0.7, 0.3, 0.3)),
        )),
        Box::new(PlainObject::new(
            Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5),
            Metal::new(Color::new(0.8, 0.8, 0.8), 0.3),
        )),
        Box::new(PlainObject::new(
            Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5),
            Metal::new(Color::new(0.8, 0.6, 0.2), 1.0),
        )),
    ]);
    (camera, objects)
}
