use std::io::Result;
use std::io::Write;

use rand::Rng as _;

use crate::camera::Camera;
use crate::color::Color;
use crate::object::Object;
use crate::ray::Ray;
use crate::rng::Rng;

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
        let (texture, maybe_new_ray) = hit.material.scatter(ray, &hit.hit, rng);
        let color = texture.color(hit.hit.u, hit.hit.v, hit.hit.point);
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
    use rand::SeedableRng;

    use crate::scene;

    use super::*;
    use crate::geom::Vec3;

    #[test]
    fn test_render_ray() {
        let mut rng = Rng::seed_from_u64(3);
        let (_, world) = scene::one_weekend::image10(1.5, &mut rng);
        let color = trace_ray(
            &Ray::new(Vec3::ZERO, Vec3::new(0.0, 0.0, -1.0), 0.0),
            &world,
            &mut rng,
            10,
        );
        eprintln!("{:?}", color);
    }
}
