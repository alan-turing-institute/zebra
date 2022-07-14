use crate::{Time};
use crate::road::{Road, Direction};

// Workaround for lack of trait upcasting coercion.
// See https://github.com/rust-lang/rust/issues/65991 for the issue
// and https://stackoverflow.com/questions/28632968/why-doesnt-rust-support-trait-object-upcasting/28664881
// for the workaround.
pub trait AsObstacle {
    fn as_obstacle(&self) -> &dyn Obstacle;
}

pub trait Obstacle : AsObstacle {

    fn get_position(&self, road: &Road, direction: &Direction) -> f32;

    fn get_speed(&self) -> f32;

    fn get_acceleration(&self) -> f32;

    fn is_active(&self, time: Time) -> bool;
}

