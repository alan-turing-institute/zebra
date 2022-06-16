use crate::{Time, ID};
use crate::time::TimeDelta;
use crate::time::TIME_RESOLUTION;
use crate::road::Crossing;
use crate::road::Road;
use crate::road::Direction;
use crate::state::State;
use crate::event_driven_sim::EventDrivenSim;
use crate::simulation::Simulation;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde_json::to_string as to_json;

const MAX_SPEED: f32 = 13.41;
const ACCELERATION_VALUE: f32 = 3.0;
const DECCELERATION_VALUE: f32 = -4.0;

#[derive(Copy,Clone)]
pub enum Action {
    Accelerate,
    Deccelerate,
    StaticSpeed
}


pub trait Vehicle {
    fn get_id(&self) -> ID;
    fn set_id(&mut self, id: ID);
    fn get_length(&self) -> f32;
    fn get_buffer_zone(&self) -> f32;
    fn get_direction(&self) -> Direction;
    fn get_position(&self) -> f32;
    fn get_speed(&self) -> f32;
    fn get_acceleration(&self) -> f32;
    fn action(&mut self, action:Action);
    fn roll_forward_by(&mut self, duration: TimeDelta);
    fn next_crossing<'a>(&'a self, road: &'a Road) -> Option<(&'a Crossing, &'a f32)>;
    fn next_vehicle<'a>(&self, vehicles: &'a Vec<Box<dyn Vehicle>>) -> Option<&'a Box<dyn Vehicle>>;
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
        state.serialize_field("position", &self.get_position())?;
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

    fn next_crossing<'a>(&'a self, road: &'a Road) -> Option<(&'a Crossing, &'a f32)>{
        
        let pairs = road.get_crossings(&self.get_direction());  
        let mut minimum_distance: f32 = std::f32::INFINITY;
        let mut next_c: &Crossing = &pairs[0].0;
        let mut next_p: &f32 = &pairs[0].1;
        let mut distance: f32 = (next_p - self.get_position()).abs();
        let mut minimum_distance: f32 = distance;
        
        for (cross, pos) in &pairs[1..] {
            next_p = pos;
            next_c = cross;
            distance = (pos - self.get_position()).abs();
            if distance < minimum_distance && pos > &self.get_position() {                
                minimum_distance = distance;
            }
        }
    
        if minimum_distance == std::f32::INFINITY {
            Option::None
        } else {
            Option::Some((next_c, next_p))
        }   
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
            if &vehicle.get_position() < &self.get_position() {
                continue;
            }
            return Option::Some(vehicle)
        }
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
        state.serialize_field("position", &self.get_position())?;
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

        assert!(test_car.get_position() > 0.0);

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
        assert_eq!(test_car.get_position(), 0.0);
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
        assert_eq!(test_car.get_position(), 0.0);
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
	        (Crossing::Zebra { cross_time: TimeDelta::from_secs(25) }, 10.0),
	        (Crossing::Zebra { cross_time: TimeDelta::from_secs(10) }, 20.0),
	    ];

        let road = Road::new(30.0f32, crossings);
        let mut sim = EventDrivenSim::new(122, 0, 60000, 0.1, 0.2, road);
        let ped_arrival_times = vec!(0);
        let veh_arrival_times = vec!(0);

        sim.set_ped_arrival_times(ped_arrival_times);
        sim.set_veh_arrival_times(veh_arrival_times);

        let next_data: (&Crossing, &f32);

        match sim.get_state().get_vehicles()[0].next_crossing(&road) {
            Some((x, y)) => next_data = (x, y),
            None => panic!("no vals"),
        }

        assert_eq!(next_data.0, &crossings[0].0);
        assert_eq!(next_data.1, &10.0);

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


