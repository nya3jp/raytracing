use std::rc::Rc;

use rand::Rng as _;

use crate::background::Background;
use crate::background::Sky;
use crate::camera::Camera;
use crate::color::Color;
use crate::geom::Vec3;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::object::Object;
use crate::object::{Objects, PlainObject};
use crate::rng::Rng;
use crate::shape::MovingSphere;
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

#[allow(dead_code)]
pub mod debug {
    use super::*;

    fn new_basic_camera(time: TimeRange, aspect_ratio: f64) -> Camera {
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

    pub fn image(
        aspect_ratio: f64,
        _rng: &mut Rng,
    ) -> (Camera, World<impl Object, impl Background>) {
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
            new_basic_camera(time, aspect_ratio),
            World::new(objects, Sky::new()),
        )
    }
}

#[allow(dead_code)]
pub mod one_weekend {
    use super::*;

    fn new_basic_camera(time: TimeRange, aspect_ratio: f64) -> Camera {
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

    pub fn image10(
        aspect_ratio: f64,
        _rng: &mut Rng,
    ) -> (Camera, World<impl Object, impl Background>) {
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
            new_basic_camera(time, aspect_ratio),
            World::new(objects, Sky::new()),
        )
    }

    pub fn image12(
        aspect_ratio: f64,
        _rng: &mut Rng,
    ) -> (Camera, World<impl Object, impl Background>) {
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
            new_basic_camera(time, aspect_ratio),
            World::new(objects, Sky::new()),
        )
    }

    pub fn image14(
        aspect_ratio: f64,
        _rng: &mut Rng,
    ) -> (Camera, World<impl Object, impl Background>) {
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
            new_basic_camera(time, aspect_ratio),
            World::new(objects, Sky::new()),
        )
    }

    pub fn image15(
        aspect_ratio: f64,
        _rng: &mut Rng,
    ) -> (Camera, World<impl Object, impl Background>) {
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
            new_basic_camera(time, aspect_ratio),
            World::new(objects, Sky::new()),
        )
    }

    pub fn image16(
        aspect_ratio: f64,
        _rng: &mut Rng,
    ) -> (Camera, World<impl Object, impl Background>) {
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
            new_basic_camera(time, aspect_ratio),
            World::new(objects, Sky::new()),
        )
    }

    pub fn image19(
        aspect_ratio: f64,
        _rng: &mut Rng,
    ) -> (Camera, World<impl Object, impl Background>) {
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
        (camera, World::new(objects, Sky::new()))
    }

    pub fn balls(
        aspect_ratio: f64,
        rng: &mut Rng,
    ) -> (Camera, World<impl Object, impl Background>) {
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
        (camera, World::new(Objects::new(balls, time), Sky::new()))
    }
}

#[allow(dead_code)]
pub mod next_week {
    use super::*;
    use crate::background::Black;
    use crate::geom::Axis;
    use crate::material::DiffuseLight;
    use crate::shape::Rectangle;

    fn random_balls(
        aspect_ratio: f64,
        rng: &mut Rng,
        checker: bool,
    ) -> (Camera, World<impl Object, impl Background>) {
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
        (camera, World::new(Objects::new(balls, time), Sky::new()))
    }

    pub fn image1(
        aspect_ratio: f64,
        rng: &mut Rng,
    ) -> (Camera, World<impl Object, impl Background>) {
        random_balls(aspect_ratio, rng, false)
    }

    pub fn image2(
        aspect_ratio: f64,
        rng: &mut Rng,
    ) -> (Camera, World<impl Object, impl Background>) {
        random_balls(aspect_ratio, rng, true)
    }

    pub fn image3(
        aspect_ratio: f64,
        _rng: &mut Rng,
    ) -> (Camera, World<impl Object, impl Background>) {
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
        (camera, World::new(objects, Sky::new()))
    }

    pub fn image13(
        aspect_ratio: f64,
        rng: &mut Rng,
    ) -> (Camera, World<impl Object, impl Background>) {
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
        (camera, World::new(objects, Sky::new()))
    }

    pub fn image15(
        aspect_ratio: f64,
        _rng: &mut Rng,
    ) -> (Camera, World<impl Object, impl Background>) {
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
        (camera, World::new(objects, Sky::new()))
    }

    pub fn image16(
        aspect_ratio: f64,
        rng: &mut Rng,
    ) -> (Camera, World<impl Object, impl Background>) {
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
        (camera, World::new(objects, Black::new()))
    }
}
