use crate::camera::Camera;
use crate::color::Color;
use crate::ray::Ray;
use crate::rng::Rng;
use crate::sampler::{MixedSampler, Sampler};
use crate::shape::{Shape, EMPTY_SHAPE};
use crate::world::World;
use anyhow::Result;
use rand::Rng as _;
use rand::SeedableRng;
use rayon::prelude::*;
use std::rc::Rc;

pub trait Writer {
    fn want_write(&self, samples: usize) -> bool;
    fn write(&self, samples: usize, pixels: &[u8]) -> Result<()>;
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
    writer: &impl Writer,
    camera: Camera,
    world: &World,
    params: RenderParams,
    samples_per_pixel: usize,
) -> Result<()> {
    let mut renderer = Renderer::new(params, camera, world);
    for i in 0..samples_per_pixel {
        renderer.trace();
        let samples = i + 1;
        if writer.want_write(samples) {
            writer.write(samples, &renderer.encode())?;
        }
    }
    Ok(())
}

#[derive(Clone, Debug)]
pub struct RenderParams {
    pub width: u32,
    pub height: u32,
    pub importance_sampling: bool,
}

pub struct Renderer<'a> {
    params: RenderParams,
    camera: Camera,
    world: &'a World,
    important: Box<dyn Shape>,
    rngs: Vec<Rng>,

    samples: usize,
    sums: Vec<Color>,
}

impl<'a> Renderer<'a> {
    pub fn new(params: RenderParams, camera: Camera, world: &'a World) -> Self {
        let important = if params.importance_sampling {
            let important = world.object.important_shape();
            eprintln!("Important: {:?}", &important);
            important
        } else {
            eprintln!("Important: <Ignored>");
            Box::new(EMPTY_SHAPE)
        };
        let sums = vec![Color::BLACK; (params.width * params.height) as usize];
        let rngs: Vec<Rng> = (0..(params.width * params.height))
            .map(|k| Rng::seed_from_u64(k as u64))
            .collect();
        Renderer {
            params,
            camera,
            world,
            important,
            rngs,
            samples: 0,
            sums,
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        self.sums
            .iter()
            .flat_map(|sum| {
                let color = (*sum / (self.samples as f64)).clamp(0.0, 1.0).gamma2();
                color.encode()
            })
            .collect()
    }

    pub fn trace(&mut self) {
        let mut rngs = std::mem::take(&mut self.rngs);
        let colors: Vec<Color> = rngs
            .par_iter_mut()
            .enumerate()
            .map(|(k, rng)| {
                let i = k % self.params.width as usize;
                let j = (self.params.height - 1) as usize - k / self.params.width as usize;
                let u = (i as f64 + rng.gen::<f64>()) / (self.params.width as f64);
                let v = (j as f64 + rng.gen::<f64>()) / (self.params.height as f64);
                let ray = self.camera.ray(u, v, rng);
                trace_ray(&ray, self.world, self.important.as_ref(), rng, 50).clamp(0.0, 1e10)
            })
            .collect();
        self.rngs = rngs;
        self.sums.iter_mut().zip(colors).for_each(|(sum, color)| {
            *sum = *sum + color;
        });
        self.samples += 1;
    }
}
