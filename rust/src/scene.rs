use std::rc::Rc;

use rand::Rng as _;

use crate::background::Background;
use crate::background::Black;
use crate::background::Sky;
use crate::camera::Camera;
use crate::color::Color;
use crate::geom::Vec3;
use crate::geom::{Axis, Box3};
use crate::material::DiffuseLight;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::object::Object;
use crate::object::{Objects, PlainObject};
use crate::rng::Rng;
use crate::shape::Box;
use crate::shape::MovingSphere;
use crate::shape::Rectangle;
use crate::shape::Sphere;
use crate::texture::SolidColor;
use crate::texture::{Checker, Image, Marble};
use crate::time::TimeRange;
use crate::world::World;
use std::f64::consts::PI;

fn v(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3::new(x, y, z)
}

fn c(r: f64, g: f64, b: f64) -> Rc<SolidColor> {
    Rc::new(SolidColor::new(Color::new(r, g, b)))
}

const ASPECT_RATIO_WIDE: f64 = 16.0 / 9.0;
const ASPECT_RATIO_SQUARE: f64 = 1.0;

#[allow(dead_code)]
pub mod debug {
    use super::*;

    fn new_basic_camera(aspect_ratio: f64, time: TimeRange) -> Camera {
        Camera::new(
            Vec3::ZERO,
            v(0.0, 0.0, -1.0),
            PI,
            aspect_ratio,
            0.0,
            1.0,
            time,
        )
    }

    pub fn image(_rng: &mut Rng) -> (f64, Camera, World<impl Object, impl Background>) {
        let aspect_ratio = ASPECT_RATIO_WIDE;
        let time = TimeRange::ZERO;
        let objects = Objects::new(
            vec![
                PlainObject::new_rc(
                    Sphere::new(v(0.0, 0.0, -1.0), 0.5),
                    Lambertian::new(c(0.3, 0.5, 1.0)),
                ),
                PlainObject::new_rc(
                    Sphere::new(v(0.0, -1000.5, -1.0), 1000.0),
                    Lambertian::new(Rc::new(Checker::new(
                        c(0.5, 0.5, 0.5),
                        SolidColor::new_rc(Color::WHITE),
                        0.5,
                    ))),
                ),
            ],
            time,
        );
        (
            aspect_ratio,
            new_basic_camera(aspect_ratio, time),
            World::new(objects, Sky::new()),
        )
    }
}

#[allow(dead_code)]
pub mod one_weekend {
    use super::*;

    fn new_basic_camera(aspect_ratio: f64, time: TimeRange) -> Camera {
        Camera::new(
            Vec3::ZERO,
            v(0.0, 0.0, -1.0),
            PI,
            aspect_ratio,
            0.0,
            1.0,
            time,
        )
    }

    pub fn image10(_rng: &mut Rng) -> (f64, Camera, World<impl Object, impl Background>) {
        let aspect_ratio = ASPECT_RATIO_WIDE;
        let time = TimeRange::ZERO;
        let objects = Objects::new(
            vec![
                PlainObject::new_rc(
                    Sphere::new(v(0.0, 0.0, -1.0), 0.5),
                    Lambertian::new(c(0.5, 0.5, 0.5)),
                ),
                PlainObject::new_rc(
                    Sphere::new(v(0.0, -100.5, -1.0), 100.0),
                    Lambertian::new(c(0.5, 0.5, 0.5)),
                ),
            ],
            time,
        );
        (
            aspect_ratio,
            new_basic_camera(aspect_ratio, time),
            World::new(objects, Sky::new()),
        )
    }

    pub fn image12(_rng: &mut Rng) -> (f64, Camera, World<impl Object, impl Background>) {
        let aspect_ratio = ASPECT_RATIO_WIDE;
        let time = TimeRange::ZERO;
        let objects = Objects::new(
            vec![
                PlainObject::new_rc(
                    Sphere::new(v(0.0, -100.5, -1.0), 100.0),
                    Lambertian::new(c(0.8, 0.8, 0.0)),
                ),
                PlainObject::new_rc(
                    Sphere::new(v(0.0, 0.0, -1.0), 0.5),
                    Lambertian::new(c(0.7, 0.3, 0.3)),
                ),
                PlainObject::new_rc(
                    Sphere::new(v(-1.0, 0.0, -1.0), 0.5),
                    Metal::new(c(0.8, 0.8, 0.8), 0.3),
                ),
                PlainObject::new_rc(
                    Sphere::new(v(1.0, 0.0, -1.0), 0.5),
                    Metal::new(c(0.8, 0.6, 0.2), 1.0),
                ),
            ],
            time,
        );
        (
            aspect_ratio,
            new_basic_camera(aspect_ratio, time),
            World::new(objects, Sky::new()),
        )
    }

    pub fn image14(_rng: &mut Rng) -> (f64, Camera, World<impl Object, impl Background>) {
        let aspect_ratio = ASPECT_RATIO_WIDE;
        let time = TimeRange::ZERO;
        let objects = Objects::new(
            vec![
                PlainObject::new_rc(
                    Sphere::new(v(0.0, -100.5, -1.0), 100.0),
                    Lambertian::new(c(0.8, 0.8, 0.0)),
                ),
                PlainObject::new_rc(
                    Sphere::new(v(0.0, 0.0, -1.0), 0.5),
                    Dielectric::new(SolidColor::new_rc(Color::WHITE), 1.5),
                ),
                PlainObject::new_rc(
                    Sphere::new(v(-1.0, 0.0, -1.0), 0.5),
                    Dielectric::new(SolidColor::new_rc(Color::WHITE), 1.5),
                ),
                PlainObject::new_rc(
                    Sphere::new(v(1.0, 0.0, -1.0), 0.5),
                    Metal::new(c(0.8, 0.6, 0.2), 1.0),
                ),
            ],
            time,
        );
        (
            aspect_ratio,
            new_basic_camera(aspect_ratio, time),
            World::new(objects, Sky::new()),
        )
    }

    pub fn image15(_rng: &mut Rng) -> (f64, Camera, World<impl Object, impl Background>) {
        let aspect_ratio = ASPECT_RATIO_WIDE;
        let time = TimeRange::ZERO;
        let objects = Objects::new(
            vec![
                PlainObject::new_rc(
                    Sphere::new(v(0.0, -100.5, -1.0), 100.0),
                    Lambertian::new(c(0.8, 0.8, 0.0)),
                ),
                PlainObject::new_rc(
                    Sphere::new(v(0.0, 0.0, -1.0), 0.5),
                    Lambertian::new(c(0.1, 0.2, 0.5)),
                ),
                PlainObject::new_rc(
                    Sphere::new(v(-1.0, 0.0, -1.0), 0.5),
                    Dielectric::new(SolidColor::new_rc(Color::WHITE), 1.5),
                ),
                PlainObject::new_rc(
                    Sphere::new(v(1.0, 0.0, -1.0), 0.5),
                    Metal::new(c(0.8, 0.6, 0.2), 0.0),
                ),
            ],
            time,
        );
        (
            aspect_ratio,
            new_basic_camera(aspect_ratio, time),
            World::new(objects, Sky::new()),
        )
    }

    pub fn image16(_rng: &mut Rng) -> (f64, Camera, World<impl Object, impl Background>) {
        let aspect_ratio = ASPECT_RATIO_WIDE;
        let time = TimeRange::ZERO;
        let objects = Objects::new(
            vec![
                PlainObject::new_rc(
                    Sphere::new(v(0.0, -100.5, -1.0), 100.0),
                    Lambertian::new(c(0.8, 0.8, 0.0)),
                ),
                PlainObject::new_rc(
                    Sphere::new(v(0.0, 0.0, -1.0), 0.5),
                    Lambertian::new(c(0.1, 0.2, 0.5)),
                ),
                PlainObject::new_rc(
                    Sphere::new(v(-1.0, 0.0, -1.0), 0.5),
                    Dielectric::new(SolidColor::new_rc(Color::WHITE), 1.5),
                ),
                PlainObject::new_rc(
                    Sphere::new(v(-1.0, 0.0, -1.0), -0.4),
                    Dielectric::new(SolidColor::new_rc(Color::WHITE), 1.5),
                ),
                PlainObject::new_rc(
                    Sphere::new(v(1.0, 0.0, -1.0), 0.5),
                    Metal::new(c(0.8, 0.6, 0.2), 0.0),
                ),
            ],
            time,
        );
        (
            aspect_ratio,
            new_basic_camera(aspect_ratio, time),
            World::new(objects, Sky::new()),
        )
    }

    pub fn image19(_rng: &mut Rng) -> (f64, Camera, World<impl Object, impl Background>) {
        let aspect_ratio = ASPECT_RATIO_WIDE;
        let time = TimeRange::ZERO;
        let objects = Objects::new(
            vec![
                PlainObject::new_rc(
                    Sphere::new(v(0.0, -100.5, -1.0), 100.0),
                    Lambertian::new(c(0.8, 0.8, 0.0)),
                ),
                PlainObject::new_rc(
                    Sphere::new(v(0.0, 0.0, -1.0), 0.5),
                    Lambertian::new(c(0.1, 0.2, 0.5)),
                ),
                PlainObject::new_rc(
                    Sphere::new(v(-1.0, 0.0, -1.0), 0.5),
                    Dielectric::new(SolidColor::new_rc(Color::WHITE), 1.5),
                ),
                PlainObject::new_rc(
                    Sphere::new(v(-1.0, 0.0, -1.0), -0.45),
                    Dielectric::new(SolidColor::new_rc(Color::WHITE), 1.5),
                ),
                PlainObject::new_rc(
                    Sphere::new(v(1.0, 0.0, -1.0), 0.5),
                    Metal::new(c(0.8, 0.6, 0.2), 0.0),
                ),
            ],
            time,
        );
        let camera = Camera::new(
            v(-2.0, 2.0, 1.0),
            v(0.0, 0.0, -1.0),
            PI / 9.0,
            aspect_ratio,
            0.0,
            1.0,
            time,
        );
        (aspect_ratio, camera, World::new(objects, Sky::new()))
    }

    pub fn balls(rng: &mut Rng) -> (f64, Camera, World<impl Object, impl Background>) {
        let aspect_ratio = ASPECT_RATIO_WIDE;
        let time = TimeRange::ZERO;
        let mut balls: Vec<Rc<dyn Object>> = vec![
            // Ground
            PlainObject::new_rc(
                Sphere::new(v(0.0, -1000.0, 0.0), 1000.0),
                Lambertian::new(c(0.5, 0.5, 0.5)),
            ),
            // Large balls
            PlainObject::new_rc(
                Sphere::new(v(0.0, 1.0, 0.0), 1.0),
                Dielectric::new(SolidColor::new_rc(Color::WHITE), 1.5),
            ),
            PlainObject::new_rc(
                Sphere::new(v(-4.0, 1.0, 0.0), 1.0),
                Lambertian::new(c(0.4, 0.2, 0.1)),
            ),
            PlainObject::new_rc(
                Sphere::new(v(4.0, 1.0, 0.0), 1.0),
                Metal::new(c(0.7, 0.6, 0.5), 0.0),
            ),
        ];
        // Small balls
        for a in -11..11 {
            for b in -11..11 {
                let center = v(
                    a as f64 + 0.9 * rng.gen::<f64>(),
                    0.2,
                    b as f64 + 0.9 * rng.gen::<f64>(),
                );
                if (center - v(4.0, 0.2, 0.0)).abs() < 0.9 {
                    continue;
                }
                let shape = Sphere::new(center, 0.2);
                let choose_mat = rng.gen::<f64>();
                balls.push(if choose_mat < 0.8 {
                    let albedo = Color::random(rng) * Color::random(rng);
                    PlainObject::new_rc(shape, Lambertian::new(SolidColor::new_rc(albedo)))
                } else if choose_mat < 0.95 {
                    let albedo = Color::random(rng) * 0.5 + Color::new(0.5, 0.5, 0.5);
                    let fuzz = rng.gen_range(0.0..0.5);
                    PlainObject::new_rc(shape, Metal::new(SolidColor::new_rc(albedo), fuzz))
                } else {
                    PlainObject::new_rc(
                        shape,
                        Dielectric::new(SolidColor::new_rc(Color::WHITE), 1.5),
                    )
                });
            }
        }
        let camera = Camera::new(
            v(13.0, 2.0, 3.0),
            Vec3::ZERO,
            PI / 9.0,
            aspect_ratio,
            0.1,
            10.0,
            time,
        );
        (
            aspect_ratio,
            camera,
            World::new(Objects::new(balls, time), Sky::new()),
        )
    }
}

#[allow(dead_code)]
pub mod next_week {
    use super::*;
    use crate::shape::{Rotate, Translate};

    fn random_balls(
        rng: &mut Rng,
        checker: bool,
    ) -> (f64, Camera, World<impl Object, impl Background>) {
        let aspect_ratio = ASPECT_RATIO_WIDE;
        let time = TimeRange::new(0.0, 1.0);
        let mut balls: Vec<Rc<dyn Object>> = vec![
            // Ground
            PlainObject::new_rc(
                Sphere::new(v(0.0, -1000.0, 0.0), 1000.0),
                Lambertian::new(if checker {
                    Rc::new(Checker::new(c(0.2, 0.3, 0.1), c(0.9, 0.9, 0.9), 0.3))
                } else {
                    c(0.5, 0.5, 0.5)
                }),
            ),
            // Large balls
            PlainObject::new_rc(
                Sphere::new(v(0.0, 1.0, 0.0), 1.0),
                Dielectric::new(SolidColor::new_rc(Color::WHITE), 1.5),
            ),
            PlainObject::new_rc(
                Sphere::new(v(-4.0, 1.0, 0.0), 1.0),
                Lambertian::new(c(0.4, 0.2, 0.1)),
            ),
            PlainObject::new_rc(
                Sphere::new(v(4.0, 1.0, 0.0), 1.0),
                Metal::new(c(0.7, 0.6, 0.5), 0.0),
            ),
        ];
        // Small balls
        for a in -11..11 {
            for b in -11..11 {
                let center = v(
                    a as f64 + 0.9 * rng.gen::<f64>(),
                    0.2,
                    b as f64 + 0.9 * rng.gen::<f64>(),
                );
                if (center - v(4.0, 0.2, 0.0)).abs() < 0.9 {
                    continue;
                }
                let shape = Sphere::new(center, 0.2);
                let choose_mat = rng.gen::<f64>();
                balls.push(if choose_mat < 0.8 {
                    let albedo = Color::random(rng) * Color::random(rng);
                    let center1 = center + v(0.0, rng.gen_range(0.0..=0.5), 0.0);
                    PlainObject::new_rc(
                        MovingSphere::new(center, center1, time, 0.2),
                        Lambertian::new(SolidColor::new_rc(albedo)),
                    )
                } else if choose_mat < 0.95 {
                    let albedo = Color::random(rng) * 0.5 + Color::new(0.5, 0.5, 0.5);
                    let fuzz = rng.gen_range(0.0..0.5);
                    PlainObject::new_rc(shape, Metal::new(SolidColor::new_rc(albedo), fuzz))
                } else {
                    PlainObject::new_rc(
                        shape,
                        Dielectric::new(SolidColor::new_rc(Color::WHITE), 1.5),
                    )
                });
            }
        }
        let camera = Camera::new(
            v(13.0, 2.0, 3.0),
            Vec3::ZERO,
            PI / 9.0,
            aspect_ratio,
            0.1,
            10.0,
            time,
        );
        (
            aspect_ratio,
            camera,
            World::new(Objects::new(balls, time), Sky::new()),
        )
    }

    pub fn image1(rng: &mut Rng) -> (f64, Camera, World<impl Object, impl Background>) {
        random_balls(rng, false)
    }

    pub fn image2(rng: &mut Rng) -> (f64, Camera, World<impl Object, impl Background>) {
        random_balls(rng, true)
    }

    pub fn image3(_rng: &mut Rng) -> (f64, Camera, World<impl Object, impl Background>) {
        let aspect_ratio = ASPECT_RATIO_WIDE;
        let time = TimeRange::ZERO;
        let checker = Rc::new(Checker::new(c(0.2, 0.3, 0.1), c(0.9, 0.9, 0.9), 0.3));
        let objects = Objects::new(
            vec![
                PlainObject::new_rc(
                    Sphere::new(v(0.0, -10.0, 0.0), 10.0),
                    Lambertian::new(checker.clone()),
                ),
                PlainObject::new_rc(
                    Sphere::new(v(0.0, 10.0, 0.0), 10.0),
                    Lambertian::new(checker.clone()),
                ),
            ],
            time,
        );
        let camera = Camera::new(
            v(13.0, 2.0, 3.0),
            Vec3::ZERO,
            PI / 9.0,
            aspect_ratio,
            0.0,
            1.0,
            time,
        );
        (aspect_ratio, camera, World::new(objects, Sky::new()))
    }

    pub fn image13(rng: &mut Rng) -> (f64, Camera, World<impl Object, impl Background>) {
        let aspect_ratio = ASPECT_RATIO_WIDE;
        let time = TimeRange::ZERO;
        let noise = Rc::new(Marble::new(4.0, rng));
        let objects = Objects::new(
            vec![
                PlainObject::new_rc(
                    Sphere::new(v(0.0, -1000.0, 0.0), 1000.0),
                    Lambertian::new(noise.clone()),
                ),
                PlainObject::new_rc(
                    Sphere::new(v(0.0, 2.0, 0.0), 2.0),
                    Lambertian::new(noise.clone()),
                ),
            ],
            time,
        );
        let camera = Camera::new(
            v(13.0, 2.0, 3.0),
            Vec3::ZERO,
            PI / 9.0,
            aspect_ratio,
            0.0,
            1.0,
            time,
        );
        (aspect_ratio, camera, World::new(objects, Sky::new()))
    }

    pub fn image15(_rng: &mut Rng) -> (f64, Camera, World<impl Object, impl Background>) {
        let aspect_ratio = ASPECT_RATIO_WIDE;
        let time = TimeRange::ZERO;
        let image = Rc::new(Image::load("third_party/earthmap.jpg").expect("failed to load image"));
        let objects = Objects::new(
            vec![PlainObject::new_rc(
                Sphere::new(v(0.0, 0.0, 0.0), 2.0),
                Lambertian::new(image.clone()),
            )],
            time,
        );
        let camera = Camera::new(
            v(13.0, 2.0, 3.0),
            Vec3::ZERO,
            PI / 9.0,
            aspect_ratio,
            0.0,
            1.0,
            time,
        );
        (aspect_ratio, camera, World::new(objects, Sky::new()))
    }

    pub fn image16(rng: &mut Rng) -> (f64, Camera, World<impl Object, impl Background>) {
        let aspect_ratio = ASPECT_RATIO_WIDE;
        let time = TimeRange::ZERO;
        let noise = Rc::new(Marble::new(4.0, rng));
        let objects = Objects::new(
            vec![
                PlainObject::new_rc(
                    Sphere::new(v(0.0, -1000.0, 0.0), 1000.0),
                    Lambertian::new(noise.clone()),
                ),
                PlainObject::new_rc(
                    Sphere::new(v(0.0, 2.0, 0.0), 2.0),
                    Lambertian::new(noise.clone()),
                ),
                PlainObject::new_rc(
                    Rectangle::new(Axis::Z, -2.0, 3.0, 5.0, 1.0, 3.0),
                    DiffuseLight::new(c(4.0, 4.0, 4.0)),
                ),
            ],
            time,
        );
        let camera = Camera::new(
            v(26.0, 3.0, 6.0),
            Vec3::new(0.0, 2.0, 0.0),
            PI / 9.0,
            aspect_ratio,
            0.0,
            1.0,
            time,
        );
        (aspect_ratio, camera, World::new(objects, Black::new()))
    }

    pub fn image18(_rng: &mut Rng) -> (f64, Camera, World<impl Object, impl Background>) {
        let aspect_ratio = ASPECT_RATIO_SQUARE;
        let time = TimeRange::ZERO;
        let red = Lambertian::new(c(0.65, 0.05, 0.05));
        let white = Lambertian::new(c(0.73, 0.73, 0.73));
        let green = Lambertian::new(c(0.12, 0.45, 0.15));
        let light = DiffuseLight::new(c(15.0, 15.0, 15.0));
        let objects = Objects::new(
            vec![
                PlainObject::new_rc(
                    Rectangle::new(Axis::X, 555.0, 0.0, 555.0, 0.0, 555.0),
                    green.clone(),
                ),
                PlainObject::new_rc(
                    Rectangle::new(Axis::X, 0.0, 0.0, 555.0, 0.0, 555.0),
                    red.clone(),
                ),
                PlainObject::new_rc(
                    Rectangle::new(Axis::Y, 554.0, 227.0, 332.0, 213.0, 343.0),
                    light.clone(),
                ),
                PlainObject::new_rc(
                    Rectangle::new(Axis::Y, 0.0, 0.0, 555.0, 0.0, 555.0),
                    white.clone(),
                ),
                PlainObject::new_rc(
                    Rectangle::new(Axis::Y, 555.0, 0.0, 555.0, 0.0, 555.0),
                    white.clone(),
                ),
                PlainObject::new_rc(
                    Rectangle::new(Axis::Z, 555.0, 0.0, 555.0, 0.0, 555.0),
                    white.clone(),
                ),
            ],
            time,
        );
        let camera = Camera::new(
            v(278.0, 278.0, -800.0),
            Vec3::new(278.0, 278.0, 0.0),
            PI * 2.0 / 9.0,
            aspect_ratio,
            0.0,
            1.0,
            time,
        );
        (aspect_ratio, camera, World::new(objects, Black::new()))
    }

    pub fn image19(_rng: &mut Rng) -> (f64, Camera, World<impl Object, impl Background>) {
        let aspect_ratio = ASPECT_RATIO_SQUARE;
        let time = TimeRange::ZERO;
        let red = Lambertian::new(c(0.65, 0.05, 0.05));
        let white = Lambertian::new(c(0.73, 0.73, 0.73));
        let green = Lambertian::new(c(0.12, 0.45, 0.15));
        let light = DiffuseLight::new(c(15.0, 15.0, 15.0));
        let objects = Objects::new(
            vec![
                PlainObject::new_rc(
                    Rectangle::new(Axis::X, 555.0, 0.0, 555.0, 0.0, 555.0),
                    green.clone(),
                ),
                PlainObject::new_rc(
                    Rectangle::new(Axis::X, 0.0, 0.0, 555.0, 0.0, 555.0),
                    red.clone(),
                ),
                PlainObject::new_rc(
                    Rectangle::new(Axis::Y, 554.0, 227.0, 332.0, 213.0, 343.0),
                    light.clone(),
                ),
                PlainObject::new_rc(
                    Rectangle::new(Axis::Y, 0.0, 0.0, 555.0, 0.0, 555.0),
                    white.clone(),
                ),
                PlainObject::new_rc(
                    Rectangle::new(Axis::Y, 555.0, 0.0, 555.0, 0.0, 555.0),
                    white.clone(),
                ),
                PlainObject::new_rc(
                    Rectangle::new(Axis::Z, 555.0, 0.0, 555.0, 0.0, 555.0),
                    white.clone(),
                ),
                PlainObject::new_rc(
                    Box::new(Box3::new(v(130.0, 0.0, 65.0), v(295.0, 165.0, 230.0))),
                    white.clone(),
                ),
                PlainObject::new_rc(
                    Box::new(Box3::new(v(265.0, 0.0, 295.0), v(430.0, 330.0, 460.0))),
                    white.clone(),
                ),
            ],
            time,
        );
        let camera = Camera::new(
            v(278.0, 278.0, -800.0),
            Vec3::new(278.0, 278.0, 0.0),
            PI * 2.0 / 9.0,
            aspect_ratio,
            0.0,
            1.0,
            time,
        );
        (aspect_ratio, camera, World::new(objects, Black::new()))
    }

    pub fn image20(_rng: &mut Rng) -> (f64, Camera, World<impl Object, impl Background>) {
        let aspect_ratio = ASPECT_RATIO_SQUARE;
        let time = TimeRange::ZERO;
        let red = Lambertian::new(c(0.65, 0.05, 0.05));
        let white = Lambertian::new(c(0.73, 0.73, 0.73));
        let green = Lambertian::new(c(0.12, 0.45, 0.15));
        let light = DiffuseLight::new(c(15.0, 15.0, 15.0));
        let objects = Objects::new(
            vec![
                PlainObject::new_rc(
                    Rectangle::new(Axis::X, 555.0, 0.0, 555.0, 0.0, 555.0),
                    green.clone(),
                ),
                PlainObject::new_rc(
                    Rectangle::new(Axis::X, 0.0, 0.0, 555.0, 0.0, 555.0),
                    red.clone(),
                ),
                PlainObject::new_rc(
                    Rectangle::new(Axis::Y, 554.0, 227.0, 332.0, 213.0, 343.0),
                    light.clone(),
                ),
                PlainObject::new_rc(
                    Rectangle::new(Axis::Y, 0.0, 0.0, 555.0, 0.0, 555.0),
                    white.clone(),
                ),
                PlainObject::new_rc(
                    Rectangle::new(Axis::Y, 555.0, 0.0, 555.0, 0.0, 555.0),
                    white.clone(),
                ),
                PlainObject::new_rc(
                    Rectangle::new(Axis::Z, 555.0, 0.0, 555.0, 0.0, 555.0),
                    white.clone(),
                ),
                PlainObject::new_rc(
                    Translate::new(
                        v(265.0, 0.0, 295.0),
                        Rotate::new(
                            Axis::Y,
                            PI / 12.0,
                            Box::new(Box3::new(v(0.0, 0.0, 0.0), v(165.0, 330.0, 165.0))),
                        ),
                    ),
                    white.clone(),
                ),
                PlainObject::new_rc(
                    Translate::new(
                        v(130.0, 0.0, 65.0),
                        Rotate::new(
                            Axis::Y,
                            -PI / 10.0,
                            Box::new(Box3::new(v(0.0, 0.0, 0.0), v(165.0, 165.0, 165.0))),
                        ),
                    ),
                    white.clone(),
                ),
            ],
            time,
        );
        let camera = Camera::new(
            v(278.0, 278.0, -800.0),
            Vec3::new(278.0, 278.0, 0.0),
            PI * 2.0 / 9.0,
            aspect_ratio,
            0.0,
            1.0,
            time,
        );
        (aspect_ratio, camera, World::new(objects, Black::new()))
    }
}
