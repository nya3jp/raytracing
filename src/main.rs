use anyhow::Result;
use clap::Clap;
use engine::{render, Rng, Scene, SceneParams, Writer};
use rand::SeedableRng;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Clap)]
struct Opts {
    #[clap(short, long)]
    width: Option<u32>,
    #[clap(short, long, default_value = "out.png")]
    output: PathBuf,
    #[clap(short, long, default_value = "book3/image12")]
    scene: String,
    #[clap(short, long)]
    samples: Option<usize>,
    #[clap(short, long, default_value = "10")]
    write_period: usize,
    #[clap(short, long)]
    importance_sampling: Option<bool>,
}

fn apply_opts(params: &mut SceneParams, opts: &Opts) {
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

struct EagerWriter {
    output: PathBuf,
    width: u32,
    height: u32,
    samples_per_pixel: usize,
    period: usize,
}

impl Writer for EagerWriter {
    fn want_write(&self, samples: usize) -> bool {
        samples == self.samples_per_pixel || samples % self.period == 0
    }

    fn write(&self, _samples: usize, pixels: &[u8]) -> Result<()> {
        let file = File::create(&self.output)?;
        let mut encoder = png::Encoder::new(BufWriter::new(file), self.width, self.height);
        encoder.set_color(png::ColorType::RGB);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?.into_stream_writer();
        writer.write_all(pixels)?;
        Ok(())
    }
}

fn main() -> Result<()> {
    const BASE_SEED: u64 = 28;

    let opts = Opts::parse();

    let scene = Scene::from_str(&opts.scene)?;

    let (mut params, camera, world) = scene.load(&mut Rng::seed_from_u64(BASE_SEED));

    apply_opts(&mut params, &opts);

    let writer = EagerWriter {
        output: opts.output,
        width: params.width,
        height: params.height,
        samples_per_pixel: params.samples_per_pixel,
        period: opts.write_period,
    };

    render(
        &writer,
        camera,
        &world,
        params.render_params(),
        params.samples_per_pixel,
    )?;

    Ok(())
}
