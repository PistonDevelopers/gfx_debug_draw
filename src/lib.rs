#[macro_use]
extern crate gfx;
extern crate gfx_text;

mod debug_renderer;
mod line_renderer;
mod utils;

pub use debug_renderer::{DebugRenderer, DebugRendererError};
