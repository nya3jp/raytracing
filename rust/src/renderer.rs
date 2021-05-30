use crate::camera::Camera;
use crate::color::Color;
use crate::ray::Ray;
use crate::rng::Rng;
use crate::sampler::{MixedSampler, Sampler};
use crate::shape::{Shape, EMPTY_SHAPE};
use crate::world::World;
use rand::Rng as _;
use rayon::prelude::*;
use std::io::Result;
use std::io::Write;
use std::rc::Rc;

pub struct RenderParams {
    pub width: u32,
    pub height: u32,
    pub samples_per_pixel: usize,
    pub importance_sampling: bool,
}

fn trace_ray(
    ray: &Ray,
    world: &World,
    important: &dyn Shape,
    rng: &mut Rng,
    limit: isize,
) -> Color {
    if limit <= 0 {
        return Color::BLACK;
    }
    if let Some(mut hit) = world.object.hit(ray, 1e-8, f64::INFINITY, rng) {
        let scatter_sampler = std::mem::replace(&mut hit.scatter.sampler, None);
        hit.scatter.emit
            + hit.scatter.albedo
                * scatter_sampler.map_or(Color::BLACK, |scatter_sampler| {
                    let scatter_sampler: Rc<dyn Sampler> = scatter_sampler.into();
                    let point = hit.scatter.point;
                    let mut trace_sampler = scatter_sampler.clone();
                    if let Some(important_sampler) = important.sampler(point, ray.time) {
                        trace_sampler = Rc::new(MixedSampler::new(vec![
                            scatter_sampler.clone(),
                            important_sampler.into(),
                        ]));
                    }
                    let (new_dir, weight) = trace_sampler.constant().map_or_else(
                        || {
                            let new_dir = trace_sampler.sample(rng);
                            (
                                new_dir,
                                scatter_sampler.probability(new_dir)
                                    / trace_sampler.probability(new_dir),
                            )
                        },
                        |new_dir| (new_dir, 1.0),
                    );
                    if weight == 0.0 {
                        return Color::BLACK;
                    }
                    weight
                        * trace_ray(
                            &Ray::new(point, new_dir, ray.time),
                            world,
                            important,
                            rng,
                            limit - 1,
                        )
                })
    } else {
        world.background.color(ray)
    }
}

pub fn render(
    writer: &mut impl Write,
    camera: &Camera,
    world: &World,
    params: &RenderParams,
    rngs: &mut Vec<Rng>,
) -> Result<()> {
    let important = if params.importance_sampling {
        let important = world.object.important_shape();
        eprintln!("Important: {:?}", &important);
        important
    } else {
        eprintln!("Important: <Ignored>");
        Box::new(EMPTY_SHAPE)
    };
    for j in (0..params.height).rev() {
        eprint!("{}/{}\n", params.height - 1 - j, params.height);
        for i in 0..params.width {
            let color = rngs
                .par_iter_mut()
                .map(|rng| {
                    let u = (i as f64 + rng.gen::<f64>()) / (params.width as f64);
                    let v = (j as f64 + rng.gen::<f64>()) / (params.height as f64);
                    let ray = camera.ray(u, v, rng);
                    trace_ray(&ray, world, important.as_ref(), rng, 50).clamp(0.0, 1e10)
                })
                .sum::<Color>()
                / params.samples_per_pixel as f64;
            let color = color.clamp(0.0, 1.0).gamma2();
            writer.write(&color.encode())?;
        }
    }
    Ok(())
}
