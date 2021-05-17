use crate::color::Color;
use crate::geom::{Ray, Vec3};
use crate::object::Object;
use crate::rng::Rng;
use rand::Rng as _;
use std::io::Result;
use std::io::Write;

#[derive(Debug)]
pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64) -> Camera {
        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;
        let focal_length = 1.0;

        let origin = Vec3::ZERO;
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin + Vec3::new(0.0, 0.0, -focal_length) - horizontal / 2.0 - vertical / 2.0;
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        let target = self.lower_left_corner + self.horizontal * u + self.vertical * v;
        Ray::new(self.origin, target - self.origin)
    }
}

fn render_sky(ray: &Ray) -> Color {
    let t = 0.5 * (ray.dir.unit().y + 1.0);
    (1.0 - t) * Color::WHITE + t * Color::new(0.5, 0.7, 1.0)
}

fn render_ray<O: Object>(ray: &Ray, world: &O, rng: &mut Rng) -> Color {
    if let Some(hit) = world.hit(ray, 1e-3, 1e10) {
        let n = hit.hit.normal;
        return 0.5 * Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
    }
    render_sky(ray)
}

const SAMPLES: u32 = 100;

pub fn render<W: Write, O: Object>(
    writer: &mut W,
    camera: &Camera,
    world: &O,
    width: u32,
    height: u32,
    rng: &mut Rng,
) -> Result<()> {
    for j in (0..height).rev() {
        print!("{}/{}\r", height - 1 - j, height);
        for i in 0..width {
            let mut sum_color = Color::BLACK;
            for _ in 0..SAMPLES {
                let u = (i as f64 + rng.gen::<f64>()) / (width as f64);
                let v = (j as f64 + rng.gen::<f64>()) / (height as f64);
                let ray = camera.ray(u, v);
                let color = render_ray(&ray, world, rng);
                sum_color = sum_color + color;
            }
            sum_color = sum_color / SAMPLES as f64;
            writer.write(&sum_color.encode())?;
        }
    }
    Ok(())
}
