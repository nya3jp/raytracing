use std::io::Result;
use std::io::Write;

use rand::Rng as _;

use crate::background::Background;
use crate::camera::Camera;
use crate::color::Color;
use crate::object::Object;
use crate::ray::Ray;
use crate::rng::Rng;
use crate::world::World;

fn trace_ray(
    ray: &Ray,
    world: &World<impl Object, impl Background>,
    rng: &mut Rng,
    limit: isize,
) -> Color {
    //eprintln!("render_ray({:?})", ray);
    if limit <= 0 {
        return Color::BLACK;
    }
    if let Some(hit) = world.object.hit(ray, 1e-8, f64::INFINITY, rng) {
        hit.scatter.emit
            + hit.scatter.ray.as_ref().map_or(Color::BLACK, |new_ray| {
                hit.scatter.attenuation * trace_ray(&new_ray, world, rng, limit - 1)
            })
    } else {
        world.background.color(ray)
    }
}

const SAMPLES: u32 = 100;

pub fn render(
    writer: &mut impl Write,
    camera: &Camera,
    world: &World<impl Object, impl Background>,
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
        let (_, _, world) = scene::one_weekend::image10(&mut rng);
        let color = trace_ray(
            &Ray::new(Vec3::ZERO, Vec3::new(0.0, 0.0, -1.0), 0.0),
            &world,
            &mut rng,
            10,
        );
        eprintln!("{:?}", color);
    }
}
