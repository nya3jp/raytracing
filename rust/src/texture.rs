use crate::color::Color;
use crate::geom::Vec3;
use crate::rng::Rng;
use rand::seq::SliceRandom;

use std::rc::Rc;

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

    pub fn new_rc(color: Color) -> Rc<Self> {
        Rc::new(Self::new(color))
    }
}

impl From<Color> for SolidColor {
    fn from(color: Color) -> Self {
        Self::new(color)
    }
}

pub struct Checker {
    even: Rc<dyn Texture>,
    odd: Rc<dyn Texture>,
    stride: f64,
}

impl Texture for Checker {
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

impl Checker {
    pub fn new(even: Rc<dyn Texture>, odd: Rc<dyn Texture>, stride: f64) -> Self {
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
