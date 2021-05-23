use crate::color::Color;
use crate::geom::Vec3;
use crate::rng::Rng;
use rand::seq::SliceRandom;

use jpeg_decoder::{ImageInfo, PixelFormat};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::{fmt, io, result};

pub trait Texture {
    fn color(&self, u: f64, v: f64, p: Vec3) -> Color;
}

#[derive(Clone, Copy, Debug)]
pub struct SolidColor(Color);

impl Texture for SolidColor {
    fn color(&self, _u: f64, _v: f64, _p: Vec3) -> Color {
        self.0
    }
}

impl SolidColor {
    pub fn new(color: Color) -> Self {
        SolidColor(color)
    }
}

impl From<Color> for SolidColor {
    fn from(color: Color) -> Self {
        Self::new(color)
    }
}

#[derive(Clone)]
pub struct Checker<A: Texture, B: Texture> {
    even: A,
    odd: B,
    stride: f64,
}

impl<A: Texture, B: Texture> Texture for Checker<A, B> {
    fn color(&self, u: f64, v: f64, p: Vec3) -> Color {
        let alt = |w: f64| (w / self.stride).rem_euclid(2.0) as isize;
        let bit = alt(p.x) ^ alt(p.y) ^ alt(p.z);
        if bit & 1 == 0 {
            self.even.color(u, v, p)
        } else {
            self.odd.color(u, v, p)
        }
    }
}

impl<A: Texture, B: Texture> Checker<A, B> {
    pub fn new(even: A, odd: B, stride: f64) -> Self {
        Checker { even, odd, stride }
    }
}

#[derive(Clone)]
pub struct Marble {
    perlin: Perlin,
    scale: f64,
}

impl Texture for Marble {
    fn color(&self, _u: f64, _v: f64, p: Vec3) -> Color {
        // Color::WHITE * ((self.perlin.noise(p * self.scale) + 1.0) * 0.5)
        Color::WHITE * (0.5 * (1.0 + (self.scale * p.z + 10.0 * self.perlin.turbulence(p)).sin()))
    }
}

impl Marble {
    pub fn new(scale: f64, rng: &mut Rng) -> Marble {
        Marble {
            perlin: Perlin::new(rng),
            scale,
        }
    }
}

#[derive(Clone)]
pub struct Image {
    pixels: Vec<u8>,
    info: ImageInfo,
}

impl Texture for Image {
    fn color(&self, u: f64, v: f64, _p: Vec3) -> Color {
        let i = ((self.info.height as f64 * (1.0 - v)) as u16).min(self.info.height - 1) as usize;
        let j = ((self.info.width as f64 * u) as u16).min(self.info.width - 1) as usize;
        let offset = (i * self.info.width as usize + j) * 3;
        fn f(b: u8) -> f64 {
            b as f64 / 255.0
        }
        Color::new(
            f(self.pixels[offset + 0]),
            f(self.pixels[offset + 1]),
            f(self.pixels[offset + 2]),
        )
    }
}

impl Image {
    pub fn load(path: impl AsRef<Path>) -> result::Result<Image, ImageError> {
        let file = File::open(path)?;
        let mut decoder = jpeg_decoder::Decoder::new(BufReader::new(file));
        let pixels = decoder.decode()?;
        let info = decoder.info().unwrap();
        if info.pixel_format != PixelFormat::RGB24 {
            return Err(ImageError::Decoder(jpeg_decoder::Error::Format(format!(
                "Unsupported pixel format: {:?}",
                info.pixel_format
            ))));
        }
        Ok(Image { pixels, info })
    }
}

#[derive(Debug)]
pub enum ImageError {
    Io(io::Error),
    Decoder(jpeg_decoder::Error),
}

impl fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImageError::Io(e) => e.fmt(f),
            ImageError::Decoder(e) => e.fmt(f),
        }
    }
}

impl Error for ImageError {}

impl From<io::Error> for ImageError {
    fn from(e: io::Error) -> Self {
        ImageError::Io(e)
    }
}

impl From<jpeg_decoder::Error> for ImageError {
    fn from(e: jpeg_decoder::Error) -> Self {
        ImageError::Decoder(e)
    }
}

#[derive(Clone)]
struct Perlin {
    vecs: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    const PERIOD: usize = 256;

    fn new(rng: &mut Rng) -> Self {
        let vecs = (0..Self::PERIOD)
            .map(|_| Vec3::random_in_unit_sphere(rng).unit())
            .collect();
        let mut gen_perm = || {
            let mut perm: Vec<usize> = (0..Self::PERIOD).collect();
            perm.shuffle(rng);
            perm
        };
        Perlin {
            vecs,
            perm_x: gen_perm(),
            perm_y: gen_perm(),
            perm_z: gen_perm(),
        }
    }

    fn noise(&self, p: Vec3) -> f64 {
        fn split(x: f64) -> (usize, f64, f64) {
            let r = x.rem_euclid(Perlin::PERIOD as f64);
            let u = r.rem_euclid(1.0);
            (r as usize, u, u * u * (3.0 - 2.0 * u))
        }
        let (i, u, uu) = split(p.x);
        let (j, v, vv) = split(p.y);
        let (k, w, ww) = split(p.z);
        let mut interp = 0.0;
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let b = self.vecs[self.perm_x[(i + di) % Self::PERIOD]
                        ^ self.perm_y[(j + dj) % Self::PERIOD]
                        ^ self.perm_z[(k + dk) % Self::PERIOD]];
                    let f = (if di == 0 { 1.0 - uu } else { uu })
                        * (if dj == 0 { 1.0 - vv } else { vv })
                        * (if dk == 0 { 1.0 - ww } else { ww });
                    interp += f * b.dot(Vec3::new(u - di as f64, v - dj as f64, w - dk as f64));
                }
            }
        }
        interp
    }

    fn turbulence(&self, mut p: Vec3) -> f64 {
        let mut f = 0.0;
        let mut weight = 1.0;
        for _ in 0..7 {
            f += weight * self.noise(p);
            p = p * 2.0;
            weight /= 2.0;
        }
        f.abs()
    }
}
