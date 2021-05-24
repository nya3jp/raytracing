use crate::background::Background;
use crate::camera::Camera;
use crate::color::Color;
use crate::geom::Vec3;
use crate::geom::{Axis, Box3};
use crate::material::DiffuseLight;
use crate::material::Fog;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::object::GlobalVolume;
use crate::object::Object;
use crate::object::ObjectPtr;
use crate::object::VolumeObject;
use crate::object::{Objects, SolidObject};
use crate::object::{RotateObject, TranslateObject};
use crate::renderer::RenderParams;
use crate::rng::Rng;
use crate::shape::Box;
use crate::shape::MovingSphere;
use crate::shape::Rectangle;
use crate::shape::Sphere;
use crate::shape::{Rotate, Translate};
use crate::texture::SolidColor;
use crate::texture::{Checker, Image, Marble};
use crate::time::TimeRange;
use crate::world::World;
use itertools::Itertools;
use rand::Rng as _;
use std::f64::consts::PI;
use std::sync::Arc;

fn v(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3::new(x, y, z)
}

fn c(r: f64, g: f64, b: f64) -> SolidColor {
    SolidColor::new(Color::new(r, g, b))
}

const RENDER_PARAMS_WIDE: RenderParams = RenderParams {
    width: 400,
    height: 225,
    samples_per_pixel: 100,
};

const RENDER_PARAMS_SQAURE: RenderParams = RenderParams {
    width: 400,
    height: 400,
    samples_per_pixel: 100,
};

const RENDER_PARAMS_ONE_WEEKEND_FINAL: RenderParams = RenderParams {
    width: 1200,
    height: 800,
    samples_per_pixel: 500,
};

const RENDER_PARAMS_NEXT_WEEK_FINAL: RenderParams = RenderParams {
    width: 800,
    height: 800,
    samples_per_pixel: 10000,
};

fn aspect_ratio(params: &RenderParams) -> f64 {
    params.width as f64 / params.height as f64
}

pub fn load(name: &str, rng: &mut Rng) -> (RenderParams, Camera, World) {
    match name {
        "one_weekend::image10" => one_weekend::image10(rng),
        "one_weekend::image12" => one_weekend::image12(rng),
        "one_weekend::image14" => one_weekend::image14(rng),
        "one_weekend::image15" => one_weekend::image15(rng),
        "one_weekend::image16" => one_weekend::image16(rng),
        "one_weekend::image19" => one_weekend::image19(rng),
        "one_weekend::balls" => one_weekend::balls(rng),
        "next_week::image1" => next_week::image1(rng),
        "next_week::image2" => next_week::image2(rng),
        "next_week::image3" => next_week::image3(rng),
        "next_week::image13" => next_week::image13(rng),
        "next_week::image15" => next_week::image15(rng),
        "next_week::image16" => next_week::image16(rng),
        "next_week::image18" => next_week::image18(rng),
        "next_week::image19" => next_week::image19(rng),
        "next_week::image20" => next_week::image20(rng),
        "next_week::image21" => next_week::image21(rng),
        "next_week::all_features" => next_week::all_features(rng),
        _ => panic!("Unknown scene name: {}", name),
    }
}

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

    pub fn image(_rng: &mut Rng) -> (RenderParams, Camera, World) {
        let params = RENDER_PARAMS_WIDE;
        let time = TimeRange::ZERO;
        let objects = Objects::new(
            vec![
                SolidObject::new_rc(
                    Sphere::new(v(0.0, 0.0, -1.0), 0.5),
                    Lambertian::new(c(0.3, 0.5, 1.0)),
                ),
                SolidObject::new_rc(
                    Sphere::new(v(0.0, -1000.5, -1.0), 1000.0),
                    Lambertian::new(Checker::new(
                        c(0.5, 0.5, 0.5),
                        SolidColor::new(Color::WHITE),
                        0.5,
                    )),
                ),
            ],
            time,
        );
        let camera = new_basic_camera(aspect_ratio(&params), time);
        (params, camera, World::new(objects, Background::SKY))
    }

    pub fn balls_above(rng: &mut Rng) -> (RenderParams, Camera, World) {
        let params = RENDER_PARAMS_WIDE;
        let time = TimeRange::ZERO;
        let mut balls: Vec<Arc<dyn Object>> = vec![
            // Ground
            SolidObject::new_rc(
                Sphere::new(v(0.0, -1000.0, 0.0), 1000.0),
                Lambertian::new(c(0.5, 0.5, 0.5)),
            ),
            // Large balls
            SolidObject::new_rc(
                Sphere::new(v(0.0, 1.0, 0.0), 1.0),
                Dielectric::new(SolidColor::new(Color::WHITE), 1.5),
            ),
            SolidObject::new_rc(
                Sphere::new(v(-4.0, 1.0, 0.0), 1.0),
                Lambertian::new(c(0.4, 0.2, 0.1)),
            ),
            SolidObject::new_rc(
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
                    SolidObject::new_rc(shape, Lambertian::new(SolidColor::new(albedo)))
                } else if choose_mat < 0.95 {
                    let albedo = Color::random(rng) * 0.5 + Color::new(0.5, 0.5, 0.5);
                    let fuzz = rng.gen_range(0.0..0.5);
                    SolidObject::new_rc(shape, Metal::new(SolidColor::new(albedo), fuzz))
                } else {
                    SolidObject::new_rc(shape, Dielectric::new(SolidColor::new(Color::WHITE), 1.5))
                });
            }
        }
        let camera = Camera::new(
            v(0.0, 50.0, 1.0),
            Vec3::ZERO,
            PI / 9.0,
            aspect_ratio(&params),
            0.0,
            10.0,
            time,
        );
        (
            params,
            camera,
            World::new(Objects::new(balls, time), Background::SKY),
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

    pub fn image10(_rng: &mut Rng) -> (RenderParams, Camera, World) {
        let params = RENDER_PARAMS_WIDE;
        let time = TimeRange::ZERO;
        let objects = Objects::new(
            vec![
                SolidObject::new_rc(
                    Sphere::new(v(0.0, 0.0, -1.0), 0.5),
                    Lambertian::new(c(0.5, 0.5, 0.5)),
                ),
                SolidObject::new_rc(
                    Sphere::new(v(0.0, -100.5, -1.0), 100.0),
                    Lambertian::new(c(0.5, 0.5, 0.5)),
                ),
            ],
            time,
        );
        let camera = new_basic_camera(aspect_ratio(&params), time);
        (params, camera, World::new(objects, Background::SKY))
    }

    pub fn image12(_rng: &mut Rng) -> (RenderParams, Camera, World) {
        let params = RENDER_PARAMS_WIDE;
        let time = TimeRange::ZERO;
        let objects = Objects::new(
            vec![
                SolidObject::new_rc(
                    Sphere::new(v(0.0, -100.5, -1.0), 100.0),
                    Lambertian::new(c(0.8, 0.8, 0.0)),
                ),
                SolidObject::new_rc(
                    Sphere::new(v(0.0, 0.0, -1.0), 0.5),
                    Lambertian::new(c(0.7, 0.3, 0.3)),
                ),
                SolidObject::new_rc(
                    Sphere::new(v(-1.0, 0.0, -1.0), 0.5),
                    Metal::new(c(0.8, 0.8, 0.8), 0.3),
                ),
                SolidObject::new_rc(
                    Sphere::new(v(1.0, 0.0, -1.0), 0.5),
                    Metal::new(c(0.8, 0.6, 0.2), 1.0),
                ),
            ],
            time,
        );
        let camera = new_basic_camera(aspect_ratio(&params), time);
        (params, camera, World::new(objects, Background::SKY))
    }

    pub fn image14(_rng: &mut Rng) -> (RenderParams, Camera, World) {
        let params = RENDER_PARAMS_WIDE;
        let time = TimeRange::ZERO;
        let objects = Objects::new(
            vec![
                SolidObject::new_rc(
                    Sphere::new(v(0.0, -100.5, -1.0), 100.0),
                    Lambertian::new(c(0.8, 0.8, 0.0)),
                ),
                SolidObject::new_rc(
                    Sphere::new(v(0.0, 0.0, -1.0), 0.5),
                    Dielectric::new(SolidColor::new(Color::WHITE), 1.5),
                ),
                SolidObject::new_rc(
                    Sphere::new(v(-1.0, 0.0, -1.0), 0.5),
                    Dielectric::new(SolidColor::new(Color::WHITE), 1.5),
                ),
                SolidObject::new_rc(
                    Sphere::new(v(1.0, 0.0, -1.0), 0.5),
                    Metal::new(c(0.8, 0.6, 0.2), 1.0),
                ),
            ],
            time,
        );
        let camera = new_basic_camera(aspect_ratio(&params), time);
        (params, camera, World::new(objects, Background::SKY))
    }

    pub fn image15(_rng: &mut Rng) -> (RenderParams, Camera, World) {
        let params = RENDER_PARAMS_WIDE;
        let time = TimeRange::ZERO;
        let objects = Objects::new(
            vec![
                SolidObject::new_rc(
                    Sphere::new(v(0.0, -100.5, -1.0), 100.0),
                    Lambertian::new(c(0.8, 0.8, 0.0)),
                ),
                SolidObject::new_rc(
                    Sphere::new(v(0.0, 0.0, -1.0), 0.5),
                    Lambertian::new(c(0.1, 0.2, 0.5)),
                ),
                SolidObject::new_rc(
                    Sphere::new(v(-1.0, 0.0, -1.0), 0.5),
                    Dielectric::new(SolidColor::new(Color::WHITE), 1.5),
                ),
                SolidObject::new_rc(
                    Sphere::new(v(1.0, 0.0, -1.0), 0.5),
                    Metal::new(c(0.8, 0.6, 0.2), 0.0),
                ),
            ],
            time,
        );
        let camera = new_basic_camera(aspect_ratio(&params), time);
        (params, camera, World::new(objects, Background::SKY))
    }

    pub fn image16(_rng: &mut Rng) -> (RenderParams, Camera, World) {
        let params = RENDER_PARAMS_WIDE;
        let time = TimeRange::ZERO;
        let objects = Objects::new(
            vec![
                SolidObject::new_rc(
                    Sphere::new(v(0.0, -100.5, -1.0), 100.0),
                    Lambertian::new(c(0.8, 0.8, 0.0)),
                ),
                SolidObject::new_rc(
                    Sphere::new(v(0.0, 0.0, -1.0), 0.5),
                    Lambertian::new(c(0.1, 0.2, 0.5)),
                ),
                SolidObject::new_rc(
                    Sphere::new(v(-1.0, 0.0, -1.0), 0.5),
                    Dielectric::new(SolidColor::new(Color::WHITE), 1.5),
                ),
                SolidObject::new_rc(
                    Sphere::new(v(-1.0, 0.0, -1.0), -0.4),
                    Dielectric::new(SolidColor::new(Color::WHITE), 1.5),
                ),
                SolidObject::new_rc(
                    Sphere::new(v(1.0, 0.0, -1.0), 0.5),
                    Metal::new(c(0.8, 0.6, 0.2), 0.0),
                ),
            ],
            time,
        );
        let camera = new_basic_camera(aspect_ratio(&params), time);
        (params, camera, World::new(objects, Background::SKY))
    }

    pub fn image19(_rng: &mut Rng) -> (RenderParams, Camera, World) {
        let params = RENDER_PARAMS_WIDE;
        let time = TimeRange::ZERO;
        let objects = Objects::new(
            vec![
                SolidObject::new_rc(
                    Sphere::new(v(0.0, -100.5, -1.0), 100.0),
                    Lambertian::new(c(0.8, 0.8, 0.0)),
                ),
                SolidObject::new_rc(
                    Sphere::new(v(0.0, 0.0, -1.0), 0.5),
                    Lambertian::new(c(0.1, 0.2, 0.5)),
                ),
                SolidObject::new_rc(
                    Sphere::new(v(-1.0, 0.0, -1.0), 0.5),
                    Dielectric::new(SolidColor::new(Color::WHITE), 1.5),
                ),
                SolidObject::new_rc(
                    Sphere::new(v(-1.0, 0.0, -1.0), -0.45),
                    Dielectric::new(SolidColor::new(Color::WHITE), 1.5),
                ),
                SolidObject::new_rc(
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
            aspect_ratio(&params),
            0.0,
            1.0,
            time,
        );
        (params, camera, World::new(objects, Background::SKY))
    }

    pub fn balls(rng: &mut Rng) -> (RenderParams, Camera, World) {
        let params = RENDER_PARAMS_ONE_WEEKEND_FINAL;
        let time = TimeRange::ZERO;
        let mut balls: Vec<Arc<dyn Object>> = vec![
            // Ground
            SolidObject::new_rc(
                Sphere::new(v(0.0, -1000.0, 0.0), 1000.0),
                Lambertian::new(c(0.5, 0.5, 0.5)),
            ),
            // Large balls
            SolidObject::new_rc(
                Sphere::new(v(0.0, 1.0, 0.0), 1.0),
                Dielectric::new(SolidColor::new(Color::WHITE), 1.5),
            ),
            SolidObject::new_rc(
                Sphere::new(v(-4.0, 1.0, 0.0), 1.0),
                Lambertian::new(c(0.4, 0.2, 0.1)),
            ),
            SolidObject::new_rc(
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
                    SolidObject::new_rc(shape, Lambertian::new(SolidColor::new(albedo)))
                } else if choose_mat < 0.95 {
                    let albedo = Color::random(rng) * 0.5 + Color::new(0.5, 0.5, 0.5);
                    let fuzz = rng.gen_range(0.0..0.5);
                    SolidObject::new_rc(shape, Metal::new(SolidColor::new(albedo), fuzz))
                } else {
                    SolidObject::new_rc(shape, Dielectric::new(SolidColor::new(Color::WHITE), 1.5))
                });
            }
        }
        let camera = Camera::new(
            v(13.0, 2.0, 3.0),
            Vec3::ZERO,
            PI / 9.0,
            aspect_ratio(&params),
            0.1,
            10.0,
            time,
        );
        (
            params,
            camera,
            World::new(Objects::new(balls, time), Background::SKY),
        )
    }
}

#[allow(dead_code)]
pub mod next_week {
    use super::*;

    fn random_balls(rng: &mut Rng, checker: bool) -> (RenderParams, Camera, World) {
        let params = RENDER_PARAMS_WIDE;
        let time = TimeRange::new(0.0, 1.0);
        let mut balls: Vec<Arc<dyn Object>> = vec![
            // Ground
            if checker {
                SolidObject::new_rc(
                    Sphere::new(v(0.0, -1000.0, 0.0), 1000.0),
                    Lambertian::new(Checker::new(c(0.2, 0.3, 0.1), c(0.9, 0.9, 0.9), 0.3)),
                )
            } else {
                SolidObject::new_rc(
                    Sphere::new(v(0.0, -1000.0, 0.0), 1000.0),
                    Lambertian::new(c(0.5, 0.5, 0.5)),
                )
            },
            // Large balls
            SolidObject::new_rc(
                Sphere::new(v(0.0, 1.0, 0.0), 1.0),
                Dielectric::new(SolidColor::new(Color::WHITE), 1.5),
            ),
            SolidObject::new_rc(
                Sphere::new(v(-4.0, 1.0, 0.0), 1.0),
                Lambertian::new(c(0.4, 0.2, 0.1)),
            ),
            SolidObject::new_rc(
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
                    SolidObject::new_rc(
                        MovingSphere::new(center, center1, time, 0.2),
                        Lambertian::new(SolidColor::new(albedo)),
                    )
                } else if choose_mat < 0.95 {
                    let albedo = Color::random(rng) * 0.5 + Color::new(0.5, 0.5, 0.5);
                    let fuzz = rng.gen_range(0.0..0.5);
                    SolidObject::new_rc(shape, Metal::new(SolidColor::new(albedo), fuzz))
                } else {
                    SolidObject::new_rc(shape, Dielectric::new(SolidColor::new(Color::WHITE), 1.5))
                });
            }
        }
        let camera = Camera::new(
            v(13.0, 2.0, 3.0),
            Vec3::ZERO,
            PI / 9.0,
            aspect_ratio(&params),
            0.1,
            10.0,
            time,
        );
        (
            params,
            camera,
            World::new(Objects::new(balls, time), Background::SKY),
        )
    }

    pub fn image1(rng: &mut Rng) -> (RenderParams, Camera, World) {
        random_balls(rng, false)
    }

    pub fn image2(rng: &mut Rng) -> (RenderParams, Camera, World) {
        random_balls(rng, true)
    }

    pub fn image3(_rng: &mut Rng) -> (RenderParams, Camera, World) {
        let params = RENDER_PARAMS_WIDE;
        let time = TimeRange::ZERO;
        let checker = Checker::new(c(0.2, 0.3, 0.1), c(0.9, 0.9, 0.9), 0.3);
        let objects = Objects::new(
            vec![
                SolidObject::new_rc(
                    Sphere::new(v(0.0, -10.0, 0.0), 10.0),
                    Lambertian::new(checker.clone()),
                ),
                SolidObject::new_rc(
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
            aspect_ratio(&params),
            0.0,
            1.0,
            time,
        );
        (params, camera, World::new(objects, Background::SKY))
    }

    pub fn image13(rng: &mut Rng) -> (RenderParams, Camera, World) {
        let params = RENDER_PARAMS_WIDE;
        let time = TimeRange::ZERO;
        let noise = Marble::new(4.0, rng);
        let objects = Objects::new(
            vec![
                SolidObject::new_rc(
                    Sphere::new(v(0.0, -1000.0, 0.0), 1000.0),
                    Lambertian::new(noise.clone()),
                ),
                SolidObject::new_rc(
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
            aspect_ratio(&params),
            0.0,
            1.0,
            time,
        );
        (params, camera, World::new(objects, Background::SKY))
    }

    pub fn image15(_rng: &mut Rng) -> (RenderParams, Camera, World) {
        let params = RENDER_PARAMS_WIDE;
        let time = TimeRange::ZERO;
        let image = Image::load("third_party/earthmap.jpg").expect("failed to load image");
        let objects = Objects::new(
            vec![SolidObject::new_rc(
                Sphere::new(v(0.0, 0.0, 0.0), 2.0),
                Lambertian::new(image.clone()),
            )],
            time,
        );
        let camera = Camera::new(
            v(13.0, 2.0, 3.0),
            Vec3::ZERO,
            PI / 9.0,
            aspect_ratio(&params),
            0.0,
            1.0,
            time,
        );
        (params, camera, World::new(objects, Background::SKY))
    }

    pub fn image16(rng: &mut Rng) -> (RenderParams, Camera, World) {
        let params = RENDER_PARAMS_WIDE;
        let time = TimeRange::ZERO;
        let noise = Marble::new(4.0, rng);
        let objects = Objects::new(
            vec![
                SolidObject::new_rc(
                    Sphere::new(v(0.0, -1000.0, 0.0), 1000.0),
                    Lambertian::new(noise.clone()),
                ),
                SolidObject::new_rc(
                    Sphere::new(v(0.0, 2.0, 0.0), 2.0),
                    Lambertian::new(noise.clone()),
                ),
                SolidObject::new_rc(
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
            aspect_ratio(&params),
            0.0,
            1.0,
            time,
        );
        (params, camera, World::new(objects, Background::BLACK))
    }

    pub fn image18(_rng: &mut Rng) -> (RenderParams, Camera, World) {
        let params = RENDER_PARAMS_SQAURE;
        let time = TimeRange::ZERO;
        let red = Lambertian::new(c(0.65, 0.05, 0.05));
        let white = Lambertian::new(c(0.73, 0.73, 0.73));
        let green = Lambertian::new(c(0.12, 0.45, 0.15));
        let light = DiffuseLight::new(c(15.0, 15.0, 15.0));
        let objects = Objects::new(
            vec![
                SolidObject::new_rc(
                    Rectangle::new(Axis::X, 555.0, 0.0, 555.0, 0.0, 555.0),
                    green.clone(),
                ),
                SolidObject::new_rc(
                    Rectangle::new(Axis::X, 0.0, 0.0, 555.0, 0.0, 555.0),
                    red.clone(),
                ),
                SolidObject::new_rc(
                    Rectangle::new(Axis::Y, 554.0, 227.0, 332.0, 213.0, 343.0),
                    light.clone(),
                ),
                SolidObject::new_rc(
                    Rectangle::new(Axis::Y, 0.0, 0.0, 555.0, 0.0, 555.0),
                    white.clone(),
                ),
                SolidObject::new_rc(
                    Rectangle::new(Axis::Y, 555.0, 0.0, 555.0, 0.0, 555.0),
                    white.clone(),
                ),
                SolidObject::new_rc(
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
            aspect_ratio(&params),
            0.0,
            1.0,
            time,
        );
        (params, camera, World::new(objects, Background::BLACK))
    }

    pub fn image19(_rng: &mut Rng) -> (RenderParams, Camera, World) {
        let params = RENDER_PARAMS_SQAURE;
        let time = TimeRange::ZERO;
        let red = Lambertian::new(c(0.65, 0.05, 0.05));
        let white = Lambertian::new(c(0.73, 0.73, 0.73));
        let green = Lambertian::new(c(0.12, 0.45, 0.15));
        let light = DiffuseLight::new(c(15.0, 15.0, 15.0));
        let objects = Objects::new(
            vec![
                SolidObject::new_rc(
                    Rectangle::new(Axis::X, 555.0, 0.0, 555.0, 0.0, 555.0),
                    green.clone(),
                ),
                SolidObject::new_rc(
                    Rectangle::new(Axis::X, 0.0, 0.0, 555.0, 0.0, 555.0),
                    red.clone(),
                ),
                SolidObject::new_rc(
                    Rectangle::new(Axis::Y, 554.0, 227.0, 332.0, 213.0, 343.0),
                    light.clone(),
                ),
                SolidObject::new_rc(
                    Rectangle::new(Axis::Y, 0.0, 0.0, 555.0, 0.0, 555.0),
                    white.clone(),
                ),
                SolidObject::new_rc(
                    Rectangle::new(Axis::Y, 555.0, 0.0, 555.0, 0.0, 555.0),
                    white.clone(),
                ),
                SolidObject::new_rc(
                    Rectangle::new(Axis::Z, 555.0, 0.0, 555.0, 0.0, 555.0),
                    white.clone(),
                ),
                SolidObject::new_rc(
                    Box::new(Box3::new(v(130.0, 0.0, 65.0), v(295.0, 165.0, 230.0))),
                    white.clone(),
                ),
                SolidObject::new_rc(
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
            aspect_ratio(&params),
            0.0,
            1.0,
            time,
        );
        (params, camera, World::new(objects, Background::BLACK))
    }

    pub fn image20(_rng: &mut Rng) -> (RenderParams, Camera, World) {
        let params = RENDER_PARAMS_SQAURE;
        let time = TimeRange::ZERO;
        let red = Lambertian::new(c(0.65, 0.05, 0.05));
        let white = Lambertian::new(c(0.73, 0.73, 0.73));
        let green = Lambertian::new(c(0.12, 0.45, 0.15));
        let light = DiffuseLight::new(c(15.0, 15.0, 15.0));
        let objects = Objects::new(
            vec![
                SolidObject::new_rc(
                    Rectangle::new(Axis::X, 555.0, 0.0, 555.0, 0.0, 555.0),
                    green.clone(),
                ),
                SolidObject::new_rc(
                    Rectangle::new(Axis::X, 0.0, 0.0, 555.0, 0.0, 555.0),
                    red.clone(),
                ),
                SolidObject::new_rc(
                    Rectangle::new(Axis::Y, 554.0, 227.0, 332.0, 213.0, 343.0),
                    light.clone(),
                ),
                SolidObject::new_rc(
                    Rectangle::new(Axis::Y, 0.0, 0.0, 555.0, 0.0, 555.0),
                    white.clone(),
                ),
                SolidObject::new_rc(
                    Rectangle::new(Axis::Y, 555.0, 0.0, 555.0, 0.0, 555.0),
                    white.clone(),
                ),
                SolidObject::new_rc(
                    Rectangle::new(Axis::Z, 555.0, 0.0, 555.0, 0.0, 555.0),
                    white.clone(),
                ),
                SolidObject::new_rc(
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
                SolidObject::new_rc(
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
            aspect_ratio(&params),
            0.0,
            1.0,
            time,
        );
        (params, camera, World::new(objects, Background::BLACK))
    }

    pub fn image21(_rng: &mut Rng) -> (RenderParams, Camera, World) {
        let params = RENDER_PARAMS_SQAURE;
        let time = TimeRange::ZERO;
        let red = Lambertian::new(c(0.65, 0.05, 0.05));
        let white = Lambertian::new(c(0.73, 0.73, 0.73));
        let green = Lambertian::new(c(0.12, 0.45, 0.15));
        let light = DiffuseLight::new(c(7.0, 7.0, 7.0));
        let objects = Objects::new(
            vec![
                SolidObject::new_rc(
                    Rectangle::new(Axis::X, 555.0, 0.0, 555.0, 0.0, 555.0),
                    green.clone(),
                ),
                SolidObject::new_rc(
                    Rectangle::new(Axis::X, 0.0, 0.0, 555.0, 0.0, 555.0),
                    red.clone(),
                ),
                SolidObject::new_rc(
                    Rectangle::new(Axis::Y, 554.0, 113.0, 443.0, 127.0, 432.0),
                    light.clone(),
                ),
                SolidObject::new_rc(
                    Rectangle::new(Axis::Y, 0.0, 0.0, 555.0, 0.0, 555.0),
                    white.clone(),
                ),
                SolidObject::new_rc(
                    Rectangle::new(Axis::Y, 555.0, 0.0, 555.0, 0.0, 555.0),
                    white.clone(),
                ),
                SolidObject::new_rc(
                    Rectangle::new(Axis::Z, 555.0, 0.0, 555.0, 0.0, 555.0),
                    white.clone(),
                ),
                VolumeObject::new_rc(
                    Translate::new(
                        v(265.0, 0.0, 295.0),
                        Rotate::new(
                            Axis::Y,
                            PI / 12.0,
                            Box::new(Box3::new(v(0.0, 0.0, 0.0), v(165.0, 330.0, 165.0))),
                        ),
                    ),
                    Fog::new(Color::BLACK),
                    0.01,
                ),
                VolumeObject::new_rc(
                    Translate::new(
                        v(130.0, 0.0, 65.0),
                        Rotate::new(
                            Axis::Y,
                            -PI / 10.0,
                            Box::new(Box3::new(v(0.0, 0.0, 0.0), v(165.0, 165.0, 165.0))),
                        ),
                    ),
                    Fog::new(Color::WHITE),
                    0.01,
                ),
            ],
            time,
        );
        let camera = Camera::new(
            v(278.0, 278.0, -800.0),
            Vec3::new(278.0, 278.0, 0.0),
            PI * 2.0 / 9.0,
            aspect_ratio(&params),
            0.0,
            1.0,
            time,
        );
        (params, camera, World::new(objects, Background::BLACK))
    }

    pub fn all_features(rng: &mut Rng) -> (RenderParams, Camera, World) {
        let params = RENDER_PARAMS_NEXT_WEEK_FINAL;
        let time = TimeRange::new(0.0, 1.0);

        // Boxes on the ground
        let floor = Objects::new(
            (0..20).cartesian_product(0..20).map(|(i, j)| {
                let w = 100.0;
                let x0 = -1000.0 + w * i as f64;
                let y0 = 0.0;
                let z0 = -1000.0 + w * j as f64;
                let x1 = x0 + w;
                let y1 = y0 + rng.gen_range(1.0..101.0);
                let z1 = z0 + w;
                SolidObject::new_rc(
                    Box::new(Box3::new(v(x0, y0, z0), v(x1, y1, z1))),
                    Lambertian::new(c(0.48, 0.83, 0.53)),
                )
            }),
            time,
        );

        let sphere_boundary = Sphere::new(v(360.0, 150.0, 145.0), 70.0);

        let middle = Objects::new(
            vec![
                // Light on the ceil
                SolidObject::new_rc(
                    Rectangle::new(Axis::Y, 554.0, 123.0, 423.0, 147.0, 412.0),
                    DiffuseLight::new(c(7.0, 7.0, 7.0)),
                ),
                // Moving sphere
                SolidObject::new_rc(
                    MovingSphere::new(v(400.0, 400.0, 200.0), v(430.0, 400.0, 200.0), time, 50.0),
                    Lambertian::new(c(0.7, 0.3, 0.1)),
                ),
                // Dielectric sphere
                SolidObject::new_rc(
                    Sphere::new(v(260.0, 150.0, 45.0), 50.0),
                    Dielectric::new(SolidColor::new(Color::WHITE), 1.5),
                ),
                // Metal sphere
                SolidObject::new_rc(
                    Sphere::new(v(0.0, 150.0, 145.0), 50.0),
                    Metal::new(c(0.8, 0.8, 0.9), 1.0),
                ),
                // Foggy sphere
                SolidObject::new_rc(
                    sphere_boundary.clone(),
                    Dielectric::new(SolidColor::new(Color::WHITE), 1.5),
                ),
                VolumeObject::new_rc(
                    sphere_boundary.clone(),
                    Fog::new(Color::new(0.2, 0.4, 0.9)),
                    0.2,
                ),
                // Earth sphere
                SolidObject::new_rc(
                    Sphere::new(v(400.0, 200.0, 400.0), 100.0),
                    Lambertian::new(Image::load("third_party/earthmap.jpg").unwrap()),
                ),
                // Marble sphere
                SolidObject::new_rc(
                    Sphere::new(v(220.0, 280.0, 300.0), 80.0),
                    Lambertian::new(Marble::new(0.1, rng)),
                ),
                // Mass balls
                Arc::new(TranslateObject::new(
                    v(-100.0, 270.0, 395.0),
                    RotateObject::new(
                        Axis::Y,
                        PI / 12.0,
                        Objects::new(
                            (0..1000).map(|_| {
                                SolidObject::new_rc(
                                    Sphere::new(
                                        v(
                                            rng.gen::<f64>() * 165.0,
                                            rng.gen::<f64>() * 165.0,
                                            rng.gen::<f64>() * 165.0,
                                        ),
                                        10.0,
                                    ),
                                    Lambertian::new(c(0.73, 0.73, 0.73)),
                                )
                            }),
                            time,
                        ),
                    ),
                )),
            ],
            time,
        );

        // Global fog
        let global_boundary = Sphere::new(v(0.0, 0.0, 0.0), 5000.0);
        let _global = Objects::new(
            vec![SolidObject::new_rc(
                global_boundary.clone(),
                Dielectric::new(SolidColor::new(Color::WHITE), 1.5),
            )],
            time,
        );

        let all = GlobalVolume::new(
            Fog::new(Color::WHITE),
            5000.0,
            0.0001,
            Objects::new(
                vec![Arc::new(floor) as ObjectPtr, Arc::new(middle) as ObjectPtr],
                time,
            ),
        );

        let camera = Camera::new(
            v(478.0, 278.0, -600.0),
            Vec3::new(278.0, 278.0, 0.0),
            PI * 2.2 / 9.0,
            aspect_ratio(&params),
            0.0,
            1.0,
            time,
        );
        (params, camera, World::new(all, Background::BLACK))
    }
}
