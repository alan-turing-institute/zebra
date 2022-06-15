use crate::Time;
use crate::time::TimeDelta;
use crate::time::TIME_RESOLUTION;
use crate::road::Direction;

const MAX_SPEED: f32 = 13.41;
const ACCELERATION_VALUE: f32 = 3.0;
const DECCELERATION_VALUE: f32 = -4.0;

pub enum Action {
    Accelerate,
    Deccelerate,
    StaticSpeed
}


pub trait Vehicle {
    fn get_length(&self) -> f32;
    fn get_buffer_zone(&self) -> f32;
    fn get_direction(&self) -> Direction;
    fn get_position(&self) -> f32;
    fn get_speed(&self) -> f32;
    fn get_acceleration(&self) -> f32;
    fn action(&mut self, action:Action);
    fn roll_forward_by(&mut self, duration: TimeDelta);
}

pub struct Car {
    length: f32,
    buffer_zone: f32,
    direction: Direction,
    position: f32,
    speed: f32,
    acceleration: f32,

}

impl Car {
    pub fn new(direction: Direction, speed: f32, action: Action) -> Car {
       let mut car = Car { position: 0.0f32,
              length: 4.0f32,
              buffer_zone: 1.0f32,
              direction,
              speed,
              acceleration: 0.0f32,
        };

        car.action(action);
        car
    }
}

impl Vehicle for Car {

    fn get_length(&self) -> f32 {
       self.length
    }
    fn get_buffer_zone(&self) -> f32 {
        self.buffer_zone
    }

    fn get_direction(&self) -> Direction {
        self.direction
    }

    fn get_position(&self) -> f32 {
        self.position
    }

    fn get_speed(&self) -> f32 {
        self.speed
    }

    fn get_acceleration(&self) -> f32 {
        self.acceleration
    }

    fn action(&mut self, action:Action) {
        match action {
            Action::Accelerate  => self.acceleration = ACCELERATION_VALUE,
            Action::Deccelerate => self.acceleration = DECCELERATION_VALUE,
            Action::StaticSpeed => self.acceleration = 0.0
        };
    }

    fn roll_forward_by(&mut self, time_delta: TimeDelta) {

        let seconds: f32 = time_delta.into();

        // Update the vehicle's position.
        self.position = self.position + self.speed * seconds + (0.5 * self.acceleration * seconds * seconds);

        // Update the vehicle's speed.
        self.speed = self.speed + self.acceleration * seconds;

        assert!(self.speed <= MAX_SPEED);
        assert!(self.speed >= 0.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_car_postion(){
        let test_car = Car::new(Direction::Up, 13.0,Action::Accelerate);
        assert_eq!(test_car.get_position(), 0.0);
    }

    #[test]
    fn test_car_direction(){
        let test_car = Car::new(Direction::Up, 13.0,Action::Accelerate);
        matches!(test_car.get_direction(), Direction::Up);
    }

    #[test]
    fn test_roll_forward_static(){
        let mut test_car = Car::new(Direction::Up, 0.0,Action::Accelerate);
        test_car.action(Action::StaticSpeed);
        test_car.roll_forward_by(TimeDelta::new(5000));
        assert_eq!(test_car.get_speed(), 0.0);
        assert_eq!(test_car.get_position(), 0.0);
        assert_eq!(test_car.get_acceleration(), 0.0);
    }

    fn test_roll_forward_acceleration(){
        let mut test_car = Car::new(Direction::Up, 13.0,Action::Accelerate);
        test_car.action(Action::Accelerate);

        let mut test_secs = TimeDelta::new(1000);
        test_car.roll_forward_by(test_secs);
        // assert_eq!(test_car.get_speed(), (test_secs as f32) * test_car.get_acceleration());

        let seconds: f32 = test_secs.into();
        assert_eq!(test_car.get_speed(), seconds * test_car.get_acceleration());
        assert!(test_car.get_position() > 0.0);
        assert_eq!(test_car.get_acceleration(), 3.0);
    }

}



    // Test cases for roll forwards
    // Position = 0, acceleration = 0, speed = 0
    // Position = 0, acceleration = +3, speed = 0
    // Position = 0, acceleration = 0, speed = 10
    // Position = 0, acceleration = +3, speed = 10
    // Position = 0, acceleration = -4, speed = 10

    // Tests for the Controller(?)
    // Don't deccelerate when speed is 0
    // Don't accelerate if at the speed limit
    // Stop deceleration when speed is 0


