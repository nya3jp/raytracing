mod color;
mod geom;
mod material;
mod object;
mod renderer;
mod rng;
mod scene;

extern crate png;

use crate::renderer::render;
use rand::SeedableRng;
use rand_pcg::Pcg64Mcg;
use std::fs::File;
use std::io::Result;

fn main() -> Result<()> {
    let image_width = 400;
    let image_height = 225;
    let aspect_ratio = (image_width as f64) / (image_height as f64);

    let file = File::create("out.png")?;
    let mut encoder = png::Encoder::new(file, image_width, image_height);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?.into_stream_writer();

    let (camera, world) = scene::sample(aspect_ratio);

    let mut rng = Pcg64Mcg::seed_from_u64(283);
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
