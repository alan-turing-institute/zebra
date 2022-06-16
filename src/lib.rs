
mod time;
mod road;
mod state;
mod pedestrian;
mod vehicle;
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

pub use road::*;
pub use config::get_zebra_config;
pub use simulation::Simulation;
