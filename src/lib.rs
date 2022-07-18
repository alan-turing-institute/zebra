
mod utils;
mod time;
mod road;
pub mod state;
mod pedestrian;
mod vehicle;
mod obstacle;
mod simulation;
mod config;
mod events;
pub mod event_driven_sim;

pub use time::TimeDelta;
pub use config::get_zebra_config;
pub use simulation::Simulation;

pub type Time = i64;
pub type ID = u64;
pub type Length = f32;
pub type Speed = f32;
pub type Acceleration = f32;
pub type Position = Length;

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

use std::io;                                                                                                                                                              
fn raw_input() -> () {                                                                                                                                                    
    let mut buffer = String::new();                                                                                                                                       
    io::stdin()                                                                                                                                                           
        .read_line(&mut buffer)                                                                                                                                           
        .map_err(|err| println!("{:?}", err))                                                                                                                             
        .ok();                                                                                                                                                            
} 
