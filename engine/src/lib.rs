mod background;
mod camera;
mod color;
mod geom;
mod material;
mod object;
mod parallel;
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

pub use renderer::{render, RenderParams, Renderer};
pub use rng::Rng;
pub use scene::{Scene, SceneParams};
