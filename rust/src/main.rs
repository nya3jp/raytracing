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
    let mut rng = Rng::seed_from_u64(28);

    let (params, camera, world) = scene::next_week::all_features(&mut rng);

    world
        .object
        .debug_object_tree(crate::time::TimeRange::ZERO, 0);

    let file = File::create("out.png")?;
    let mut encoder = png::Encoder::new(BufWriter::new(file), params.width, params.height);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?.into_stream_writer();

    render(&mut writer, &camera, &world, &params, &mut rng)?;

    Ok(())
}
