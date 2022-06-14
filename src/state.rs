use std::time::{Duration, Instant};
use crate::pedestrian::Pedestrian;
use crate::Time;

trait Vehicle {

}

trait State {

    // fn get_vehicles(&self) -> &[dyn Vehicle];
    fn timestamp(&self) -> Instant;

    // get the list of vehicles
    fn get_vehicles(&self) -> &Vec<Box<dyn Vehicle>>;

    // get the list of pedestrians
    fn get_pedestrians(&self) ->  &Vec<Pedestrian>;

    // get time interval until next event
    fn time_to_next_event(&self, ped_arrival_times: &[Time], veh_arrival_times: &[Time]) -> Duration; // **NOTE** new parameters.

    // roll state forward by time interval
    fn roll_forward_by(&mut self, duration: Duration);

    // update state
    fn instantaneous_update(&mut self);

}


struct SimulatorState<'a> {

    vehicles: Vec<Box<dyn Vehicle>>,
    pedestrians: Vec<Pedestrian<'a>>,
}

impl<'a> SimulatorState<'a> {

    // Constructor for the initial state at time 0.
    pub fn new() -> SimulatorState<'a> {

        SimulatorState {vehicles: Vec::new(), pedestrians: Vec::new()}
    }
}

impl<'a> State for SimulatorState<'a> {

    fn timestamp(&self) -> Instant {
        Instant::now()
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
    fn time_to_next_event(&self, ped_arrival_times: &[Time], veh_arrival_times: &[Time]) -> Duration {
        Duration::new(0, 0)
    }

    // roll state forward by time interval
    fn roll_forward_by(&mut self, duration: Duration) {

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
}