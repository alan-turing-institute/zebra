use crate::Time;
use crate::time::TimeDelta;
use crate::vehicle::{Vehicle, Car, Action};
use std::collections::VecDeque;
use crate::road::{Direction, Crossing};
use crate::pedestrian::Pedestrian;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde_json::to_string as to_json;

pub trait State {

    // fn get_vehicles(&self) -> &[dyn Vehicle];
    fn timestamp(&self) -> &Time;

    // get the list of vehicles
    fn get_vehicles(&self) -> &VecDeque<Box<dyn Vehicle>>;

    // get the list of pedestrians
    fn get_pedestrians(&self) ->  &VecDeque<Pedestrian>;

    fn get_vehicle(&self, idx: usize) -> &dyn Vehicle;
    fn get_mut_vehicle(&mut self, idx: usize) -> &mut dyn Vehicle;
    fn get_pedestrian(&self, idx: usize) -> &Pedestrian;
    fn get_mut_pedestrian(&mut self, idx: usize) -> &mut Pedestrian<'_>;

    fn push_pedestrian(&mut self, pedestrian: Pedestrian<'_>) -> usize;
    fn pop_pedestrian(&mut self, idx: usize) -> Pedestrian<'_>;
    fn push_vehicle(&mut self, vehicle: Box<dyn Vehicle>) -> usize;
    fn pop_vehicle(&mut self, idx: usize) -> Box<dyn Vehicle>;
    // MOVED TO THE SIMULATION TRAIT:
    // // get time interval until next event
    // fn time_to_next_event(&self, ped_arrival_times: &[Time], veh_arrival_times: &[Time]) -> TimeDelta;

    // // roll state forward by time interval
    // fn roll_forward_by(&mut self, time_delta: TimeDelta);

    // // update state
    // fn instantaneous_update(&mut self);

}

impl Serialize for dyn State {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Number of fields in the struct and name.
        let mut state = serializer.serialize_struct("State", 3)?;
        state.serialize_field("timestamp", &self.timestamp())?;
        state.serialize_field("pedestrians", &self.get_pedestrians())?;
        state.serialize_field("vehicles", &self.get_vehicles())?;
        state.end()
    }
}


pub struct SimulatorState<'a> {

    vehicles: VecDeque<Box<dyn Vehicle>>,
    pedestrians: VecDeque<Pedestrian<'a>>,
    timestamp: Time
}

impl Serialize for SimulatorState <'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Number of fields in the struct and name.
        let mut state = serializer.serialize_struct("State", 3)?;
        state.serialize_field("timestamp", &self.timestamp)?;
        state.serialize_field("pedestrians", &self.get_pedestrians())?;
        state.serialize_field("vehicles", &self.vehicles)?;
        state.end()
    }
}

impl<'a> SimulatorState<'a> {

    // Constructor for the initial state at time 0.
    pub fn new() -> SimulatorState<'a> {

        SimulatorState {vehicles: VecDeque::new(), pedestrians: VecDeque::new(), timestamp: 0}
    }

    // Construct a state with arbitrary content
    pub fn dummy(vehicles: VecDeque<Box<dyn Vehicle>>,
        pedestrians: VecDeque<Pedestrian<'a>>,
        timestamp: Time) -> SimulatorState<'a> {

        SimulatorState{vehicles, pedestrians, timestamp}
    }
}

impl<'a> State for SimulatorState<'a> {

    fn timestamp(&self) -> &Time {
        &self.timestamp
    }

    // get the list of vehicles
    fn get_vehicles(&self) -> &VecDeque<Box<dyn Vehicle>> {
        &self.vehicles
    }


    // get the list of pedestrians
    fn get_pedestrians(&self) ->  &VecDeque<Pedestrian> {
        &self.pedestrians
    }

    fn get_vehicle(&self, idx: usize) -> &dyn Vehicle {
        todo!()
    }

    fn get_mut_vehicle(&mut self, idx: usize) -> &mut dyn Vehicle {
        todo!()
    }

    fn get_pedestrian(&self, idx: usize) -> &Pedestrian {
        todo!()
    }

    fn get_mut_pedestrian(&mut self, idx: usize) -> &mut Pedestrian<'_> {
        todo!()
    }

    fn push_pedestrian(&mut self, pedestrian: Pedestrian<'_>) -> usize {
        todo!()
    }

    fn pop_pedestrian(&mut self, idx: usize) -> Pedestrian<'_> {
        todo!()
    }

    fn push_vehicle(&mut self, vehicle: Box<dyn Vehicle>) -> usize {
        todo!()
    }

    fn pop_vehicle(&mut self, idx: usize) -> Box<dyn Vehicle> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulator_state_constructor() {

        let state = SimulatorState::new();

        // assert_eq!(state.timestamp(), 0.0); // Initial timestamp is zero.

        assert_eq!(state.get_vehicles().len(), 0); // No vehicles
        assert_eq!(state.get_pedestrians().len(), 0); // No pedestrians
    }

    #[test]
    fn test_simulator_state_serialize() {        
        // Make test state
        let mut test_state = SimulatorState::new();

        // Make test cars
        let car1 = Car::new(1, Direction::Up, 13.0,Action::Accelerate);
        let car2 = Car::new(2, Direction::Down, 10.0,Action::Accelerate);

        // Make test crossing
        let test_pelican = Crossing::pelican(0);

        // Make test pedestrians
        let ped1 = Pedestrian::new(1, &test_pelican, 0);
        let ped2 = Pedestrian::new(2, &test_pelican, 20);

        // Make ped_vec and veh_vec
        let ped_vec: Vec<Pedestrian> = vec![ped1, ped2];
        let veh_vec: Vec<Box<dyn Vehicle>> = vec![car1, car2]
            .drain(..)
            .map(|car| Box::<dyn Vehicle>::from(Box::new(car)))
            .collect();

        // Assign ped_vec and veh_vec to state
        test_state.pedestrians = ped_vec.into();
        test_state.vehicles = veh_vec.into();

        let as_json= to_json(&test_state).unwrap();
        println!("{}", &as_json);
        assert_eq!(&as_json, "{\"timestamp\":0,\"pedestrians\":[{\"id\":1,\"location\":0,\"arrival_time\":0},{\"id\":2,\"location\":0,\"arrival_time\":20}],\"vehicles\":[{\"id\":1,\"length\":4.0,\"buffer_zone\":1.0,\"direction\":\"Up\",\"position\":0.0,\"speed\":13.0,\"acceleration\":3.0},{\"id\":2,\"length\":4.0,\"buffer_zone\":1.0,\"direction\":\"Down\",\"position\":0.0,\"speed\":10.0,\"acceleration\":3.0}]}");
    }

}
