
use crate::road::{Road, Direction};

pub trait Obstacle {

    fn get_position(&self, road: &Road, direction: &Direction) -> f32;

    fn get_speed(&self) -> f32;

    fn get_acceleration(&self) -> f32;
}

