use crate::pedestrian::Pedestrian;
use crate::Time;
use crate::time::TimeDelta;
use crate::vehicle::{Vehicle, Car, Action};
use crate::road::Direction;

pub trait State {

    // fn get_vehicles(&self) -> &[dyn Vehicle];
    fn timestamp(&self) -> &Time;

    // get the list of vehicles
    fn get_vehicles(&self) -> &Vec<Box<dyn Vehicle>>;

    // get the list of pedestrians
    fn get_pedestrians(&self) ->  &Vec<Pedestrian>;

    // MOVED TO THE SIMULATION TRAIT:
    // // get time interval until next event
    // fn time_to_next_event(&self, ped_arrival_times: &[Time], veh_arrival_times: &[Time]) -> TimeDelta;

    // // roll state forward by time interval
    // fn roll_forward_by(&mut self, time_delta: TimeDelta);

    // // update state
    // fn instantaneous_update(&mut self);

}


pub struct SimulatorState<'a> {

    vehicles: Vec<Box<dyn Vehicle>>,
    pedestrians: Vec<Pedestrian<'a>>,
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
