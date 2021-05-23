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

use crate::renderer::render;
use crate::rng::Rng;
use rand::SeedableRng;
use std::fs::File;
use std::io::{BufWriter, Result};

fn main() -> Result<()> {
    let mut rng = Rng::seed_from_u64(283);

    let (aspect_ratio, camera, world) = scene::next_week::image20(&mut rng);

    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    let file = File::create("out.png")?;
    let mut encoder = png::Encoder::new(BufWriter::new(file), image_width, image_height);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?.into_stream_writer();

    render(
        &mut writer,
        &camera,
        &world,
        image_width,
        image_height,
        &mut rng,
    )?;

    Ok(())
}
