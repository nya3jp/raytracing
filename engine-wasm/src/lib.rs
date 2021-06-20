use engine::{Rng, Scene};
use rand::SeedableRng;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::IntoEnumIterator;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RenderParams {
    scene_name: String,
    width: u32,
    height: u32,
    samples_per_pixel: usize,
    importance_sampling: bool,
}

#[wasm_bindgen]
impl RenderParams {
    pub fn new(
        scene_name: String,
        width: u32,
        height: u32,
        samples_per_pixel: usize,
        importance_sampling: bool,
    ) -> Self {
        RenderParams {
            scene_name,
            width,
            height,
            samples_per_pixel,
            importance_sampling,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn scene_name(&self) -> String {
        self.scene_name.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn width(&self) -> u32 {
        self.width
    }
    #[wasm_bindgen(getter)]
    pub fn height(&self) -> u32 {
        self.height
    }
    #[wasm_bindgen(getter)]
    pub fn samples_per_pixel(&self) -> usize {
        self.samples_per_pixel
    }
    #[wasm_bindgen(getter)]
    pub fn importance_sampling(&self) -> bool {
        self.importance_sampling
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

const BASE_SEED: u64 = 28;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Array<RenderParams>")]
    pub type RenderParamsArray;
}

const ALL_SUPPORTED_SCENES: &'static [Scene] = &[
    Scene::Book1Image10,
    Scene::Book1Image12,
    Scene::Book1Image14,
    Scene::Book1Image15,
    Scene::Book1Image16,
    Scene::Book1Image19,
    Scene::Book1Final,
    Scene::Book3Image8,
    Scene::Book3Image9,
    Scene::Book3Image12,
    Scene::DebugGlassSphere,
    Scene::DebugPortal,
];

#[wasm_bindgen]
pub fn get_all_scenes() -> RenderParamsArray {
    let params: Vec<RenderParams> = ALL_SUPPORTED_SCENES
        .iter()
        .map(|scene| {
            let (params, _, _) = scene.load(&mut Rng::seed_from_u64(BASE_SEED));
            RenderParams {
                scene_name: scene.to_string(),
                width: params.width,
                height: params.height,
                samples_per_pixel: params.samples_per_pixel,
                importance_sampling: params.importance_sampling,
            }
        })
        .collect();
    JsValue::from_serde(&params)
        .unwrap()
        .unchecked_into::<RenderParamsArray>()
}

#[wasm_bindgen]
pub fn render(params: RenderParams) -> RenderResult {
    let scene = Scene::from_str(&params.scene_name).expect("no such scene");
    let (_, camera, world) = scene.load(&mut Rng::seed_from_u64(BASE_SEED));

    let params = engine::RenderParams {
        width: params.width,
        height: params.height,
        samples_per_pixel: params.samples_per_pixel,
        importance_sampling: params.importance_sampling,
    };

    let mut buf: Vec<u8> = vec![];
    let mut rngs: Vec<Rng> = (0..params.samples_per_pixel)
        .map(|i| Rng::seed_from_u64(BASE_SEED + i as u64))
        .collect();

    engine::render(&mut buf, &camera, &world, &params, &mut rngs).expect("render failed");

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
