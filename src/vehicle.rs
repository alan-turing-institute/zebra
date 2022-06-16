use crate::Time;
use crate::time::{TimeDelta,TIME_RESOLUTION};
use crate::road::{Road, Direction};
use crate::state::State;
use crate::obstacle::Obstacle;

const MAX_SPEED: f32 = 13.41;
const ACCELERATION_VALUE: f32 = 3.0;
const DECCELERATION_VALUE: f32 = -4.0;

#[derive(Copy,Clone)]
pub enum Action {
    Accelerate,
    Deccelerate,
    StaticSpeed
}


pub trait Vehicle : Obstacle {
    fn get_length(&self) -> f32;
    fn get_buffer_zone(&self) -> f32;
    fn get_direction(&self) -> Direction;
    fn get_veh_position(&self) -> f32;
    fn action(&mut self, action:Action);
    fn roll_forward_by(&mut self, duration: TimeDelta);
    fn relative_speed(&self, obstacle: &dyn Obstacle) -> f32;
    fn relative_position(&self, obstacle: &dyn Obstacle, road: &Road) -> f32;
    fn relative_veh_position(&self, vehicle: &dyn Vehicle) -> f32;
    fn relative_acceleration(&self, obstacle: &dyn Obstacle) -> f32;
    fn next_vehicle<'a>(&self, vehicles: &'a Vec<Box<dyn Vehicle>>) -> Option<&'a Box<dyn Vehicle>>;
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


impl Obstacle for Car{

    fn get_position(&self, road: &Road) -> f32 {
        self.position
    }

    fn get_speed(&self) -> f32 {
        self.speed
    }

    fn get_acceleration(&self) -> f32 {
        self.acceleration
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

    fn get_veh_position(&self) -> f32 {
        self.position
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

    fn relative_position(&self, obstacle: &dyn Obstacle, road: &Road)-> f32 {

        // Negative relative_position means obstacle in front of car
        let relative_position: f32 = &self.get_veh_position() - obstacle.get_position(road);

        // We should never need to have a positive relative position (looking behind)
        assert!(relative_position <= 0.0);

        relative_position
    }

    fn relative_veh_position(&self, vehicle: &dyn Vehicle) -> f32 {

        // Negative relative_position means obstacle in front of car
        let relative_position: f32 = &self.get_veh_position() - vehicle.get_veh_position();

        // We should never need to have a positive relative position (looking behind)
        assert!(relative_position <= 0.0);

        relative_position
    }

    fn relative_speed(&self,obstacle: &dyn Obstacle)-> f32 {

        // Positive relative_speed means obstacle is faster than car
        &self.get_speed() - obstacle.get_speed()
    }

    fn relative_acceleration(&self, obstacle: &dyn Obstacle) -> f32 {

        &self.get_acceleration() - obstacle.get_acceleration()
    }

    fn next_vehicle<'a>(&self, vehicles: &'a Vec<Box<dyn Vehicle>>) -> Option<&'a Box<dyn Vehicle>> {

        let my_direction = &self.get_direction();
        if vehicles.len() == 0 {
            return Option::None
        }
        for vehicle in vehicles {
            // Ignore vehicles going in the other direction.
            if matches!(vehicle.get_direction(), my_direction) {
                continue;
            }
            // TODO. TEST &/or FIX THIS!
            // WARNING!!
            // This assumes vehicles are ordered by increasing position.
            // But they might not be!
            if &vehicle.get_veh_position() < &self.get_veh_position() {
                continue;
            }
            return Option::Some(vehicle)
        }
        Option::None
    }
}

#[cfg(test)]

fn spawn_car_take_action(init_action:Action, init_speed:f32){
    let mut test_car = Car::new(Direction::Up, init_speed,init_action);

    let mut test_secs = TimeDelta::new(1000);
    test_car.roll_forward_by(test_secs);

    let seconds: f32 = test_secs.into();
    assert_eq!(test_car.get_speed(), init_speed + seconds * test_car.get_acceleration());

    assert!(test_car.get_veh_position() > 0.0);

    if matches!(init_action, Action::Accelerate){
        assert_eq!(test_car.get_acceleration(), ACCELERATION_VALUE);
    } else if matches!(init_action, Action::Deccelerate){
        assert_eq!(test_car.get_acceleration(), DECCELERATION_VALUE);
    }

}

mod tests {
    use super::*;

    #[test]
    fn test_car_postion(){
        let test_car = Car::new(Direction::Up, 13.0,Action::Accelerate);
        assert_eq!(test_car.get_veh_position(), 0.0);
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
        assert_eq!(test_car.get_veh_position(), 0.0);
        assert_eq!(test_car.get_acceleration(), 0.0);
    }

    #[test]
    fn test_roll_forward_acceleration(){
        spawn_car_take_action(Action::Accelerate, 0.0);
    }

    // IMP TODO: Test next_vehicle()

    #[test]
    fn test_roll_forward_deceleration(){
        spawn_car_take_action(Action::Deccelerate, MAX_SPEED);
    }


    #[test]
    fn test_car_as_obstacle(){

        // Subject car is going slower than the obstacle.
        let mut test_car = Car::new(Direction::Up, 5.0, Action::StaticSpeed);
        let mut test_obstacle_car = Car::new(Direction::Up, 10.0, Action::StaticSpeed);

        // Roll both cars forward 5s.
        test_car.roll_forward_by(TimeDelta::new(5000));
        test_obstacle_car.roll_forward_by(TimeDelta::new(5000));

        let relative_position: f32 = test_car.relative_veh_position(&test_obstacle_car);
        let relative_speed: f32 = test_car.relative_speed(&test_obstacle_car);

        assert_eq!(relative_position, -25.0);
        assert_eq!(relative_speed, -5.0);
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


