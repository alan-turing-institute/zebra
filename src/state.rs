use crate::pedestrian::Pedestrian;
use crate::Time;
use crate::time::TimeDelta;

trait Vehicle {

}

trait State {

    // fn get_vehicles(&self) -> &[dyn Vehicle];
    fn timestamp(&self) -> &Time;

    // get the list of vehicles
    fn get_vehicles(&self) -> &Vec<Box<dyn Vehicle>>;

    // get the list of pedestrians
    fn get_pedestrians(&self) ->  &Vec<Pedestrian>;

    // get time interval until next event
    fn time_to_next_event(&self, ped_arrival_times: &[Time], veh_arrival_times: &[Time]) -> TimeDelta; // **NOTE** new parameters.

    // roll state forward by time interval
    fn roll_forward_by(&mut self, duration: TimeDelta);

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
    fn dummy(vehicles: Vec<Box<dyn Vehicle>>, pedestrians: Vec<Pedestrian<'a>>, timestamp: Time) -> SimulatorState<'a> {
        
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
        TimeDelta::new(0)
    }

    // roll state forward by time interval
    fn roll_forward_by(&mut self, duration: TimeDelta) {

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

        let vehicles = vec!(Vehicle.new());

        let state = SimulatorState::dummy();

        let ped_arrival_times = vec!(5000, 7000);
        let veh_arrival_times = vec!(4000, 15000);

        let actual = state.time_to_next_event(&ped_arrival_times, &veh_arrival_times);

        assert_eq!(actual, TimeDelta::new(4000));

    }
}