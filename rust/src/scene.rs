use crate::color::Color;
use crate::geom::Vec3;
use crate::material::Lambertian;
use crate::object::{Objects, PlainObject, Sphere};
use crate::renderer::Camera;

pub fn sample(aspect_ratio: f64) -> (Camera, Objects) {
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
