use crate::pedestrian::Pedestrian;
use crate::Time;
use crate::time::TimeDelta;
use crate::vehicle::{Vehicle, Car, Action};
use crate::road::Direction;
use std::collections::VecDeque;

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


pub struct SimulatorState<'a> {

    vehicles: VecDeque<Box<dyn Vehicle>>,
    pedestrians: VecDeque<Pedestrian<'a>>,
    timestamp: Time
}

impl<'a> SimulatorState<'a> {

    // Constructor for the initial state at time 0.
    pub fn new() -> SimulatorState<'a> {

        SimulatorState {vehicles: Vec::new(), pedestrians: Vec::new(), timestamp: 0}
    }

    // Construct a state with arbitrary content
    pub fn dummy(vehicles: Vec<Box<dyn Vehicle>>,
        pedestrians: Vec<Pedestrian<'a>>,
        timestamp: Time) -> SimulatorState<'a> {

        SimulatorState{vehicles, pedestrians, timestamp}
    }
}

impl<'a> State for SimulatorState<'a> {

    fn timestamp(&self) -> &Time {
        &self.timestamp
    }

    // get the list of vehicles
    fn get_vehicles(&self) -> &Vec<Box<dyn Vehicle>> {
        &self.vehicles
    }

    fn get_mut_vehicles(&mut self) -> &mut Vec<Box<dyn Vehicle>>;

    // get the list of pedestrians
    fn get_pedestrians(&self) ->  &Vec<Pedestrian> {
        &self.pedestrians
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

}
