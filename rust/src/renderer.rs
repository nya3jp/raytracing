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
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        origin: Vec3,
        look_at: Vec3,
        fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let viewport_height = 2.0 * (fov / 2.0).atan();
        let viewport_width = viewport_height * aspect_ratio;

        let w = (look_at - origin).unit();
        let up = Vec3::new(0.0, 1.0, 0.0);
        let u = w.cross(up).unit();
        let v = u.cross(w);

        let horizontal = u * (focus_dist * viewport_width);
        let vertical = v * (focus_dist * viewport_height);
        let lower_left_corner = origin + w * focus_dist - horizontal / 2.0 - vertical / 2.0;
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn ray(&self, u: f64, v: f64, rng: &mut Rng) -> Ray {
        let lens = Vec3::random_in_unit_disc(rng) * self.lens_radius;
        let blur = self.u * lens.x + self.v * lens.y;
        let origin = self.origin + blur;
        let target = self.lower_left_corner + self.horizontal * u + self.vertical * v;
        Ray::new(origin, target - origin)
    }
}

fn render_sky(ray: &Ray) -> Color {
    let t = 0.5 * (ray.dir.unit().y + 1.0);
    (1.0 - t) * Color::WHITE + t * Color::new(0.5, 0.7, 1.0)
}

fn trace_ray<O: Object>(ray: &Ray, world: &O, rng: &mut Rng, limit: isize) -> Color {
    //eprintln!("render_ray({:?})", ray);
    if limit <= 0 {
        return Color::BLACK;
    }
    if let Some(hit) = world.hit(ray, 1e-3, 1e10) {
        let (color, maybe_new_ray) = hit.material.scatter(ray, &hit.hit, rng);
        if let Some(new_ray) = maybe_new_ray {
            return color * trace_ray(&new_ray, world, rng, limit - 1);
        }
        return color;
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
        eprint!("{}/{}\n", height - 1 - j, height);
        for i in 0..width {
            let mut sum_color = Color::BLACK;
            for _ in 0..SAMPLES {
                let u = (i as f64 + rng.gen::<f64>()) / (width as f64);
                let v = (j as f64 + rng.gen::<f64>()) / (height as f64);
                let ray = camera.ray(u, v, rng);
                let color = trace_ray(&ray, world, rng, 50);
                sum_color = sum_color + color;
            }
            let final_color = (sum_color / SAMPLES as f64).gamma2();
            writer.write(&final_color.encode())?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene;
    use rand::SeedableRng;

    #[test]
    fn test_render_ray() {
        let mut rng = Rng::seed_from_u64(3);
        let (_, world) = scene::one_weekend::image10(1.5);
        let color = trace_ray(
            &Ray::new(Vec3::ZERO, Vec3::new(0.0, 0.0, -1.0)),
            &world,
            &mut rng,
            10,
        );
        eprintln!("{:?}", color);
    }
}
