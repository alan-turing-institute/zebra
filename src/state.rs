use std::time::{Duration, Instant};
use crate::road::Road;
use crate::pedestrian::Pedestrian;

trait Vehicle {

}

trait State {

    // const ROAD_LENGTH: f32;
    // const CROSSINGS: Vec<Crossing>;


    // fn get_vehicles(&self) -> &[dyn Vehicle];
    fn timestamp(&self) -> Instant;

    // get the road
    fn get_road(&self) -> Road;

    // get the list of vehicles
    fn get_vehicles(&self) -> Vec<Box<dyn Vehicle>>;

    // get the list of pedestrians
    fn get_pedestrians(&self) ->  Vec<Pedestrian>;

    // get time interval until next event
    fn time_to_next_event(&self) -> Duration;

    // roll state forward by time interval
    fn roll_forward_by(&mut self, duration: Duration);

    // update state
    fn instantaneous_update(&mut self);

}


struct SimulatorState {

    road: Road,
    vehicles: Vec<Box<dyn Vehicle>>,
    pedestrians: Vec<Pedestrian>,
}

impl SimulatorState {

    // Constructor for the initial state at time 0.
    pub fn new(road: Road) -> SimulatorState {

        SimulatorState {road, vehicles: Vec::new(), pedestrians: Vec::new()}
    }
}

impl State for SimulatorState {

    fn timestamp() {

    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulator_state_constructor() {

        let road = Road { length: 20.0f32, crossings: Vec::new() };
        let state = SimulatorState::new(road);

        assert_eq!(road.get_length(), 20.0f32);

        assert_eq!(state.timestamp(), 0.0); // Initial timestamp is zero.

        assert_eq!(state.get_vehicles().length(), 0); // No vehicles
        assert_eq!(state.get_pedestrians().length(), 0); // No pedestrians
    }
}