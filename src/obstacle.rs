
use crate::road::Road;

pub trait Obstacle {
    fn get_position(&self, road: &Road) -> f32;
    fn get_speed(&self) -> f32;
    fn get_acceleration(&self) -> f32;
}

