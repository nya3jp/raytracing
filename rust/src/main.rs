mod background;
mod camera;
mod color;
mod geom;
mod material;
mod object;
mod ray;
mod renderer;
mod rng;
mod scene;
mod shape;
mod texture;
mod time;
mod world;

extern crate png;

use crate::object::Object;
use crate::renderer::render;
use crate::rng::Rng;
use rand::SeedableRng;
use std::fs::File;
use std::io::{BufWriter, Result};

fn main() -> Result<()> {
    const BASE_SEED: u64 = 28;

    let (params, camera, world) =
        scene::next_week::all_features(&mut Rng::seed_from_u64(BASE_SEED));

    world
        .object
        .debug_object_tree(crate::time::TimeRange::ZERO, 0);

    let file = File::create("out.png")?;
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
