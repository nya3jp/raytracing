use engine::{scene, Rng};
use rand::SeedableRng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub struct RenderParams {
    pub width: u32,
    pub height: u32,
    pub samples_per_pixel: usize,
    pub importance_sampling: bool,
}

#[wasm_bindgen]
impl RenderParams {
    pub fn new(
        width: u32,
        height: u32,
        samples_per_pixel: usize,
        importance_sampling: bool,
    ) -> Self {
        RenderParams {
            width,
            height,
            samples_per_pixel,
            importance_sampling,
        }
    }
}

impl Into<engine::RenderParams> for RenderParams {
    fn into(self) -> engine::RenderParams {
        engine::RenderParams {
            width: self.width,
            height: self.height,
            samples_per_pixel: self.samples_per_pixel,
            importance_sampling: self.importance_sampling,
        }
    }
}

impl From<engine::RenderParams> for RenderParams {
    fn from(p: engine::RenderParams) -> Self {
        Self {
            width: p.width,
            height: p.height,
            samples_per_pixel: p.samples_per_pixel,
            importance_sampling: p.importance_sampling,
        }
    }
}

#[wasm_bindgen]
pub struct RenderResult {
    width: u32,
    height: u32,
    buf: Vec<u8>,
}

#[wasm_bindgen]
impl RenderResult {
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn data(&self) -> *const u8 {
        self.buf.as_ptr()
    }
}

#[wasm_bindgen]
pub fn render(scene_name: &str, params: RenderParams) -> RenderResult {
    const BASE_SEED: u64 = 28;
    let (_, camera, world) = scene::load(scene_name, &mut Rng::seed_from_u64(BASE_SEED));
    let mut buf: Vec<u8> = vec![];
    let mut rngs: Vec<Rng> = (0..params.samples_per_pixel)
        .map(|i| Rng::seed_from_u64(BASE_SEED + i as u64))
        .collect();
    engine::render(&mut buf, &camera, &world, &params.into(), &mut rngs).expect("render failed");
    RenderResult {
        width: params.width,
        height: params.height,
        buf,
    }
}

#[wasm_bindgen]
pub fn install_panic_hook() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}
