mod background;
mod camera;
mod color;
mod geom;
mod material;
mod object;
mod physics;
mod ray;
mod renderer;
mod rng;
mod sampler;
mod scene;
mod shape;
mod texture;
mod time;
mod world;

extern crate png;

use crate::renderer::{render, RenderParams};
use crate::rng::Rng;
use clap::Clap;
use rand::SeedableRng;
use rayon::ThreadPoolBuilder;
use std::fs::File;
use std::io::{BufWriter, Result};
use std::path::PathBuf;

#[derive(Clap)]
struct Opts {
    #[clap(short, long)]
    width: Option<u32>,
    #[clap(short, long, default_value = "out.png")]
    output: PathBuf,
    #[clap(short, long, default_value = "rest_of_life::image12")]
    scene: String,
    #[clap(short, long)]
    samples: Option<usize>,
    #[clap(short, long, default_value = "1")]
    threads: usize,
    #[clap(short, long)]
    importance_sampling: Option<bool>,
}

fn apply_opts(params: &mut RenderParams, opts: &Opts) {
    if let Some(override_width) = opts.width {
        let old_width = params.width;
        let old_height = params.height;
        params.width = override_width;
        params.height = override_width * old_height / old_width;
    }
    if let Some(samples) = opts.samples {
        params.samples_per_pixel = samples;
    }
    if let Some(importance_sampling) = opts.importance_sampling {
        params.importance_sampling = importance_sampling;
    }
}

fn main() -> Result<()> {
    const BASE_SEED: u64 = 28;

    let opts = Opts::parse();

    ThreadPoolBuilder::new()
        .num_threads(opts.threads)
        .build_global()
        .expect("Failed to initialize thread pool");

    let (mut params, camera, world) = scene::load(&opts.scene, &mut Rng::seed_from_u64(BASE_SEED));

    apply_opts(&mut params, &opts);

    let file = File::create(opts.output)?;
    let mut encoder = png::Encoder::new(BufWriter::new(file), params.width, params.height);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?.into_stream_writer();

    let mut rngs: Vec<Rng> = (0..params.samples_per_pixel)
        .map(|i| Rng::seed_from_u64(BASE_SEED + i as u64))
        .collect();
    render(&mut writer, &camera, &world, &params, &mut rngs)?;

    Ok(())
}
