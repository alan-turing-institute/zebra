
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

pub type Time = i64;
pub type ID = u64;
pub type Length = f32;
pub type Speed = f32;
pub type Acceleration = f32;
pub type Position = Length;

pub use road::*;
pub use config::{get_zebra_config, get_zebra_config_option};
pub use simulation::Simulation;

use std::io;
fn raw_input() -> () {
    let mut buffer = String::new();                                                                                                                                       
    io::stdin()                                                                                                                                                           
        .read_line(&mut buffer)                                                                                                                                           
        .map_err(|err| println!("{:?}", err))                                                                                                                             
        .ok();                                                                                                                                                            
} 