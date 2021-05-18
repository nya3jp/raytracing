use crate::camera::Camera;
use crate::color::Color;
use crate::geom::Vec3;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::object::{Objects, PlainObject, Sphere};
use rand::Rng as _;

#[allow(dead_code)]
pub mod one_weekend {
    use super::*;
    use crate::object::Object;
    use crate::rng::Rng;
    use std::f64::consts::PI;

    fn new_basic_camera(aspect_ratio: f64) -> Camera {
        Camera::new(
            Vec3::ZERO,
            Vec3::new(0.0, 0.0, -1.0),
            PI,
            aspect_ratio,
            0.0,
            1.0,
        )
    }

    pub fn image10(aspect_ratio: f64) -> (Camera, Objects) {
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
        (new_basic_camera(aspect_ratio), objects)
    }

    pub fn image12(aspect_ratio: f64) -> (Camera, Objects) {
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
        (new_basic_camera(aspect_ratio), objects)
    }

    pub fn image14(aspect_ratio: f64) -> (Camera, Objects) {
        let objects = Objects::new(vec![
            Box::new(PlainObject::new(
                Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0),
                Lambertian::new(Color::new(0.8, 0.8, 0.0)),
            )),
            Box::new(PlainObject::new(
                Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5),
                Dielectric::new(1.5),
            )),
            Box::new(PlainObject::new(
                Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5),
                Dielectric::new(1.5),
            )),
            Box::new(PlainObject::new(
                Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5),
                Metal::new(Color::new(0.8, 0.6, 0.2), 1.0),
            )),
        ]);
        (new_basic_camera(aspect_ratio), objects)
    }

    pub fn image15(aspect_ratio: f64) -> (Camera, Objects) {
        let objects = Objects::new(vec![
            Box::new(PlainObject::new(
                Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0),
                Lambertian::new(Color::new(0.8, 0.8, 0.0)),
            )),
            Box::new(PlainObject::new(
                Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5),
                Lambertian::new(Color::new(0.1, 0.2, 0.5)),
            )),
            Box::new(PlainObject::new(
                Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5),
                Dielectric::new(1.5),
            )),
            Box::new(PlainObject::new(
                Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5),
                Metal::new(Color::new(0.8, 0.6, 0.2), 0.0),
            )),
        ]);
        (new_basic_camera(aspect_ratio), objects)
    }

    pub fn image16(aspect_ratio: f64) -> (Camera, Objects) {
        let objects = Objects::new(vec![
            Box::new(PlainObject::new(
                Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0),
                Lambertian::new(Color::new(0.8, 0.8, 0.0)),
            )),
            Box::new(PlainObject::new(
                Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5),
                Lambertian::new(Color::new(0.1, 0.2, 0.5)),
            )),
            Box::new(PlainObject::new(
                Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5),
                Dielectric::new(1.5),
            )),
            Box::new(PlainObject::new(
                Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.4),
                Dielectric::new(1.5),
            )),
            Box::new(PlainObject::new(
                Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5),
                Metal::new(Color::new(0.8, 0.6, 0.2), 0.0),
            )),
        ]);
        (new_basic_camera(aspect_ratio), objects)
    }

    pub fn image19(aspect_ratio: f64) -> (Camera, Objects) {
        let objects = Objects::new(vec![
            Box::new(PlainObject::new(
                Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0),
                Lambertian::new(Color::new(0.8, 0.8, 0.0)),
            )),
            Box::new(PlainObject::new(
                Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5),
                Lambertian::new(Color::new(0.1, 0.2, 0.5)),
            )),
            Box::new(PlainObject::new(
                Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5),
                Dielectric::new(1.5),
            )),
            Box::new(PlainObject::new(
                Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.45),
                Dielectric::new(1.5),
            )),
            Box::new(PlainObject::new(
                Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5),
                Metal::new(Color::new(0.8, 0.6, 0.2), 0.0),
            )),
        ]);
        let camera = Camera::new(
            Vec3::new(-2.0, 2.0, 1.0),
            Vec3::new(0.0, 0.0, -1.0),
            PI / 9.0,
            aspect_ratio,
            0.0,
            1.0,
        );
        (camera, objects)
    }

    pub fn balls(aspect_ratio: f64, rng: &mut Rng) -> (Camera, Objects) {
        let mut balls: Vec<Box<dyn Object>> = vec![
            // Ground
            Box::new(PlainObject::new(
                Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0),
                Lambertian::new(Color::new(0.5, 0.5, 0.5)),
            )),
            // Large balls
            Box::new(PlainObject::new(
                Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0),
                Dielectric::new(1.5),
            )),
            Box::new(PlainObject::new(
                Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0),
                Lambertian::new(Color::new(0.4, 0.2, 0.1)),
            )),
            Box::new(PlainObject::new(
                Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0),
                Metal::new(Color::new(0.7, 0.6, 0.5), 0.0),
            )),
        ];
        // Small balls
        for a in -11..11 {
            for b in -11..11 {
                let center = Vec3::new(
                    a as f64 + 0.9 * rng.gen::<f64>(),
                    0.2,
                    b as f64 + 0.9 * rng.gen::<f64>(),
                );
                if (center - Vec3::new(4.0, 0.2, 0.0)).abs() < 0.9 {
                    continue;
                }
                let shape = Sphere::new(center, 0.2);
                let choose_mat = rng.gen::<f64>();
                balls.push(if choose_mat < 0.8 {
                    let albedo = Color::random(rng) * Color::random(rng);
                    Box::new(PlainObject::new(shape, Lambertian::new(albedo)))
                } else if choose_mat < 0.95 {
                    let albedo = Color::random(rng) * 0.5 + Color::new(0.5, 0.5, 0.5);
                    let fuzz = rng.gen_range(0.0..0.5);
                    Box::new(PlainObject::new(shape, Metal::new(albedo, fuzz)))
                } else {
                    Box::new(PlainObject::new(shape, Dielectric::new(1.5)))
                });
            }
        }
        let camera = Camera::new(
            Vec3::new(13.0, 2.0, 3.0),
            Vec3::ZERO,
            PI / 9.0,
            aspect_ratio,
            0.1,
            10.0,
        );
        (camera, Objects::new(balls))
    }
}
