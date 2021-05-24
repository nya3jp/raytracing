use crate::background::Background;
use crate::camera::Camera;
use crate::color::Color;
use crate::object::Object;
use crate::ray::Ray;
use crate::rng::Rng;
use crate::world::World;
use rand::Rng as _;
use rayon::prelude::*;
use std::io::Result;
use std::io::Write;

pub struct RenderParams {
    pub width: u32,
    pub height: u32,
    pub samples_per_pixel: usize,
}

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

pub fn render(
    writer: &mut impl Write,
    camera: &Camera,
    world: &World<impl Object, impl Background>,
    params: &RenderParams,
    rngs: &mut Vec<Rng>,
) -> Result<()> {
    for j in (0..params.height).rev() {
        eprint!("{}/{}\n", params.height - 1 - j, params.height);
        for i in 0..params.width {
            let color = rngs
                .par_iter_mut()
                .map(|rng| {
                    let u = (i as f64 + rng.gen::<f64>()) / (params.width as f64);
                    let v = (j as f64 + rng.gen::<f64>()) / (params.height as f64);
                    let ray = camera.ray(u, v, rng);
                    trace_ray(&ray, world, rng, 50)
                })
                .sum::<Color>()
                / params.samples_per_pixel as f64;
            let color = color.gamma2();
            writer.write(&color.encode())?;
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
