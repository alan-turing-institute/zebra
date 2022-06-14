
mod time;
mod road;
mod state;
mod pedestrian;
mod vehicle;
mod simulation;

pub use time::TimeDelta;

pub type Time = i64;
pub type Length = f32;
pub type Speed = f32;
pub type Acceleration = f32;

pub use road::*;
