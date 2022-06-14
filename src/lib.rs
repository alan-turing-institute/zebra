mod utils;
mod time;
mod road;
mod state;
mod pedestrian;
//mod simulation;

pub use time::TimeDelta;

pub type Time = i64;
pub type Length = f32;
pub type Speed = f32;
pub type Acceleration = f32;

pub use road::*;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-zebra!");
}
