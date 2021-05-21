use crate::camera::Camera;
use crate::color::Color;
use crate::geom::Vec3;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::object::{Objects, PlainObject, Sphere};
use crate::texture::SolidColor;
use rand::Rng as _;
use std::rc::Rc;

fn v(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3::new(x, y, z)
}

fn c(r: f64, g: f64, b: f64) -> Rc<SolidColor> {
    Rc::new(SolidColor::new(Color::new(r, g, b)))
}

#[allow(dead_code)]
pub mod debug {
    use super::*;

    use crate::rng::Rng;
    use crate::texture::Checker;
    use crate::time::TimeRange;
    use std::f64::consts::PI;

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

    pub fn image(aspect_ratio: f64, _rng: &mut Rng) -> (Camera, Objects) {
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
        (new_basic_camera(time, aspect_ratio), objects)
    }
}

#[allow(dead_code)]
pub mod one_weekend {
    use super::*;
    use crate::object::Object;
    use crate::rng::Rng;
    use crate::time::TimeRange;
    use std::f64::consts::PI;

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

    pub fn image10(aspect_ratio: f64, _rng: &mut Rng) -> (Camera, Objects) {
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
        (new_basic_camera(time, aspect_ratio), objects)
    }

    pub fn image12(aspect_ratio: f64, _rng: &mut Rng) -> (Camera, Objects) {
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
        (new_basic_camera(time, aspect_ratio), objects)
    }

    pub fn image14(aspect_ratio: f64, _rng: &mut Rng) -> (Camera, Objects) {
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
        (new_basic_camera(time, aspect_ratio), objects)
    }

    pub fn image15(aspect_ratio: f64, _rng: &mut Rng) -> (Camera, Objects) {
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
        (new_basic_camera(time, aspect_ratio), objects)
    }

    pub fn image16(aspect_ratio: f64, _rng: &mut Rng) -> (Camera, Objects) {
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
        (new_basic_camera(time, aspect_ratio), objects)
    }

    pub fn image19(aspect_ratio: f64, _rng: &mut Rng) -> (Camera, Objects) {
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
        (camera, objects)
    }

    pub fn balls(aspect_ratio: f64, rng: &mut Rng) -> (Camera, Objects) {
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
        (camera, Objects::new(balls, time))
    }
}

#[allow(dead_code)]
pub mod next_week {
    use super::*;
    use crate::object::{MovingSphere, Object};
    use crate::rng::Rng;
    use crate::texture::Checker;
    use crate::time::TimeRange;
    use std::f64::consts::PI;
    use std::rc::Rc;

    fn random_balls(aspect_ratio: f64, rng: &mut Rng, checker: bool) -> (Camera, Objects) {
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
        (camera, Objects::new(balls, time))
    }

    pub fn image1(aspect_ratio: f64, rng: &mut Rng) -> (Camera, Objects) {
        random_balls(aspect_ratio, rng, false)
    }

    pub fn image2(aspect_ratio: f64, rng: &mut Rng) -> (Camera, Objects) {
        random_balls(aspect_ratio, rng, true)
    }
}
