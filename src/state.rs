use crate::pedestrian::Pedestrian;
use crate::Time;
use crate::time::TimeDelta;
use crate::vehicle::{Vehicle, Car};
use crate::road::Direction;

pub trait State {

    // fn get_vehicles(&self) -> &[dyn Vehicle];
    fn timestamp(&self) -> &Time;

    // get the list of vehicles
    fn get_vehicles(&self) -> &Vec<Box<dyn Vehicle>>;

    // get the list of pedestrians
    fn get_pedestrians(&self) ->  &Vec<Pedestrian>;

    // get time interval until next event
    fn time_to_next_event(&self, ped_arrival_times: &[Time], veh_arrival_times: &[Time]) -> TimeDelta;

    // roll state forward by time interval
    fn roll_forward_by(&mut self, time_delta: TimeDelta);

    // update state
    fn instantaneous_update(&mut self);

}


struct SimulatorState<'a> {

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
    fn dummy(vehicles: Vec<Box<dyn Vehicle>>,
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

    // get time interval until next event
    fn time_to_next_event(&self, ped_arrival_times: &[Time], veh_arrival_times: &[Time]) -> TimeDelta {
	// get min of pedestrian and vehicle arrival times
	let min_ped_times = *ped_arrival_times.iter().min().unwrap();
	let min_veh_times = *veh_arrival_times.iter().min().unwrap();

	// return the smallest of the two times as the next event
	if min_veh_times < min_ped_times {
	    return TimeDelta::new(min_veh_times);
	}
	TimeDelta::new(min_ped_times)
    }

    // roll state forward by time interval
    fn roll_forward_by(&mut self, time_delta: TimeDelta) {

    }

    // update state
    fn instantaneous_update(&mut self) {

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
    fn test_pedestrian_arrival_event() {

        let state = SimulatorState::new();

        let ped_arrival_times = vec!(10000, 20000);
        let veh_arrival_times = vec!(12000, 21000);

        let actual = state.time_to_next_event(&ped_arrival_times, &veh_arrival_times);

        assert_eq!(actual, TimeDelta::new(10000));

    }

    #[test]
    fn test_vehicle_arrival_event() {

        let state = SimulatorState::new();

        let ped_arrival_times = vec!(5000, 7000);
        let veh_arrival_times = vec!(4000, 15000);

        let actual = state.time_to_next_event(&ped_arrival_times, &veh_arrival_times);

        assert_eq!(actual, TimeDelta::new(4000));

    }

    #[test]
    fn test_vehicle_stopping_event() {

        let vehicles = vec!(Car::new(0.0,Direction::Up));

        // TODO.
        // let state = SimulatorState::dummy();

    }
}
