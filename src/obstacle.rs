pub trait Obstacle {
    fn get_position(&self) -> f32;
    fn get_speed(&self) -> f32;
    fn get_acceleration(&self) -> f32;
}

