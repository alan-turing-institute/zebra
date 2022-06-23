use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde_json::to_string as to_json;
use std::collections::VecDeque;

use crate::{Time, ID};
use crate::time::TimeDelta;
use crate::time::TIME_RESOLUTION;
use crate::road::Crossing;
use crate::road::Road;
use crate::road::Direction;
use crate::state::State;
use crate::event_driven_sim::EventDrivenSim;
use crate::simulation::Simulation;
use crate::obstacle::{Obstacle, AsObstacle};

pub const MAX_SPEED: f32 = 13.41;
pub const ACCELERATION_VALUE: f32 = 3.0;
pub const DECCELERATION_VALUE: f32 = -4.0;

#[derive(Copy,Clone)]
pub enum Action {
    Accelerate,
    Deccelerate,
    StaticSpeed
}


pub trait Vehicle : Obstacle {
    fn get_id(&self) -> ID;
    fn set_id(&mut self, id: ID);
    fn get_length(&self) -> f32;
    fn get_buffer_zone(&self) -> f32;
    fn get_direction(&self) -> Direction;
    fn get_veh_position(&self) -> f32;
    fn action(&mut self, action:Action);
    fn roll_forward_by(&mut self, duration: TimeDelta);
    fn next_crossing<'a>(&'a self, road: &'a Road) -> Option<(&'a Crossing, &'a f32)>;
    fn relative_speed(&self, obstacle: &dyn Obstacle) -> f32;
    fn relative_position(&self, obstacle: &dyn Obstacle, road: &Road) -> f32;
    fn relative_veh_position(&self, vehicle: &dyn Vehicle) -> f32;
    fn relative_acceleration(&self, obstacle: &dyn Obstacle) -> f32;
    fn next_vehicle<'a>(&self, vehicles: &'a VecDeque<Box<dyn Vehicle>>) -> Option<&'a Box<dyn Vehicle>>;
}

impl Serialize for dyn Vehicle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Number of fields in the struct and name.
        let mut state = serializer.serialize_struct("Car", 7)?;
        state.serialize_field("id", &self.get_id())?;
        state.serialize_field("length", &self.get_length())?;
        state.serialize_field("buffer_zone", &self.get_buffer_zone())?;
        state.serialize_field("direction", &self.get_direction())?;
        state.serialize_field("position", &self.get_veh_position())?;
        state.serialize_field("speed", &self.get_speed())?;
        state.serialize_field("acceleration", &self.get_acceleration())?;
        state.end()
    }
}

pub struct Car {
    id: ID,
    length: f32,
    buffer_zone: f32,
    direction: Direction,
    position: f32,
    speed: f32,
    acceleration: f32,
}

impl Car {
    pub fn new(id: ID, direction: Direction, speed: f32, action: Action) -> Car {
	let mut car = Car {
	    id: id,
	    position: 0.0f32,
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

    fn get_position(&self, road: &Road, direction: &Direction) -> f32 {

        // Check that the direction matches my direction.
        let my_direction = &self.get_direction();
        assert!(matches!(direction, my_direction));
        self.position
    }

    fn get_speed(&self) -> f32 {
        self.speed
    }

    fn get_acceleration(&self) -> f32 {
        self.acceleration
    }
}

impl<T: Obstacle> AsObstacle for T {
    fn as_osbstacle(&self) -> &dyn Obstacle {
        self
    }
}

impl Vehicle for Car {
    fn set_id(&mut self, id: ID) {
	self.id = id;
    }
    fn get_id(&self) -> ID {
	self.id
    }
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

        let mut seconds: f32 = time_delta.into();

        // If acceleration is negative, check if the time to stop is less than
        // the time to roll_forward_by. If so, set seconds as this time instead.
        if self.acceleration < 0. {
            let time_to_zero = -self.speed/self.acceleration;
            if time_to_zero < seconds {
                seconds = time_to_zero;
            }
        }

        // Update the vehicle's position.
        self.position = self.position + self.speed * seconds + (0.5 * self.acceleration * seconds * seconds);

        // Update the vehicle's speed.
        self.speed = self.speed + self.acceleration * seconds;

        assert!(self.speed <= MAX_SPEED);
        assert!(self.speed >= 0.0);
    }

    fn relative_position(&self, obstacle: &dyn Obstacle, road: &Road)-> f32 {

        // Negative relative_position means obstacle in front of car
        let relative_position: f32 = &self.get_veh_position() - obstacle.get_position(road, &self.get_direction());

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

    fn next_crossing<'a>(&'a self, road: &'a Road) -> Option<(&'a Crossing, &'a f32)>{

        let my_direction = &self.get_direction();
        let crossings = road.get_crossings(my_direction);

        if crossings.len() == 0 {
            return Option::None
        }

        for (crossing, position) in crossings {

            // This assumes crossings are ordered by increasing position.
            if &crossing.get_position(&road, my_direction) < &self.get_veh_position() {
                continue;
            }
            println!("{:?}", crossing);
            return Option::Some((crossing, position))
        }

        // If this vehicle has passed all crossings, return None.
        Option::None
    }

    fn next_vehicle<'a>(&self, vehicles: &'a VecDeque<Box<dyn Vehicle>>) -> Option<&'a Box<dyn Vehicle>> {

        let my_direction = &self.get_direction();
        if vehicles.len() == 0 {
            return Option::None
        }
        for vehicle in vehicles {
            // Ignore vehicles going in the other direction.
            if matches!(vehicle.get_direction(), my_direction) {
                continue;
            }

            // This assumes vehicles are ordered by increasing position.
            if &vehicle.get_veh_position() < &self.get_veh_position() {
                continue;
            }
            return Option::Some(vehicle)
        }

        // If this vehicle is in front of all the others, return None.
        Option::None
    }
}

impl Serialize for Car {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Number of fields in the struct and name.
        let mut state = serializer.serialize_struct("Car", 7)?;
        state.serialize_field("id", &self.get_id())?;
        state.serialize_field("length", &self.get_length())?;
        state.serialize_field("buffer_zone", &self.get_buffer_zone())?;
        state.serialize_field("direction", &self.get_direction())?;
        state.serialize_field("position", &self.get_veh_position())?;
        state.serialize_field("speed", &self.get_speed())?;
        state.serialize_field("acceleration", &self.get_acceleration())?;
        state.end()
    }
}

#[cfg(test)]

fn spawn_car_take_action(init_action:Action, init_speed:f32){
    let mut test_car = Car::new(0, Direction::Up, init_speed,init_action);

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
    fn test_get_car_id(){
        let test_car = Car::new(1, Direction::Up, 13.0,Action::Accelerate);
        assert_eq!(test_car.get_id(), 1);
    }

    #[test]
    fn test_serialize_car(){
        let test_car = Car::new(1, Direction::Up, 13.0,Action::Accelerate);
        let as_json= to_json(&test_car).unwrap();
        // println!("{}", &as_json);
        assert_eq!(&as_json, "{\"id\":1,\"length\":4.0,\"buffer_zone\":1.0,\"direction\":\"Up\",\"position\":0.0,\"speed\":13.0,\"acceleration\":3.0}");
    }

    #[test]
    fn test_car_postion(){
        let test_car = Car::new(0, Direction::Up, 13.0,Action::Accelerate);
        assert_eq!(test_car.get_veh_position(), 0.0);
    }

    #[test]
    fn test_car_direction(){
        let test_car = Car::new(0, Direction::Up, 13.0,Action::Accelerate);
        matches!(test_car.get_direction(), Direction::Up);
    }

    #[test]
    fn test_roll_forward_static(){
        let mut test_car = Car::new(0, Direction::Up, 0.0,Action::Accelerate);
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
    fn test_next_crossing(){

        let crossings = vec![
	        (Crossing::Zebra { id: 0, cross_time: TimeDelta::from_secs(25) }, 10.0),
	        (Crossing::Zebra { id: 1, cross_time: TimeDelta::from_secs(10) }, 20.0),
	    ];

        let road = Road::new(30.0f32, crossings);
        // let mut sim = EventDrivenSim::new(122, 0, 60000, 0.1, 0.2, road);
        // let ped_arrival_times = vec!(0);
        // let veh_arrival_times = vec!(0);

        // sim.set_ped_arrival_times(ped_arrival_times);
        // sim.set_veh_arrival_times(veh_arrival_times);

        // let next_data: (&Crossing, &f32);

        // match sim.get_state().get_vehicles()[0].next_crossing(sim.get_road()) {
        //     Some((x, y)) => next_data = (x, y),
        //     None => panic!("no vals"),
        // }

        // assert_eq!(next_data.0, &sim.get_road().get_crossings(&Direction::Up)[0].0);
        // assert_eq!(next_data.1, &10.0);

        let vehicle = Car::new(0, Direction::Up, 0.0, Action::Accelerate);
        let actual = vehicle.next_crossing(&road).unwrap();

        // assert_eq!(actual.0, &road.get_crossings(&Direction::Up)[0].0);
        assert_eq!(actual.1, &10.0);
    }

    #[test]
    fn test_car_as_obstacle(){

        // Subject car is going slower than the obstacle.
        let mut test_car = Car::new(0, Direction::Up, 5.0, Action::StaticSpeed);
        let mut test_obstacle_car = Car::new(0, Direction::Up, 10.0, Action::StaticSpeed);

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
