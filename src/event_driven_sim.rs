use core::ffi::c_int;
use rand::{SeedableRng}; // SeedableRng needed for the seed_from_u64 method.
use rand::rngs::StdRng;
use crate::events::{Event, EventResult, EventType};
use crate::pedestrian::Person;

use crate::Time;
use crate::time::TimeDelta;
use crate::simulation::{Simulation, arrival_times};
use crate::vehicle::{self, Vehicle, Car, Action};
use crate::road::Road;
use crate::state::{State, SimulatorState};

pub struct EventDrivenSim {

    seed: u64,

    start_time: Time,
    end_time: Time,

    ped_arrival_rate: f32,
    veh_arrival_rate: f32,

    pub ped_arrival_times: Vec<Time>,
    ped_arrival_idx: usize,
    pub veh_arrival_times: Vec<Time>,
    veh_arrival_idx: usize,

    road: Road,
    state: Box<dyn State>
}

impl EventDrivenSim {

    pub fn new(seed: u64,
        start_time: Time,
        end_time: Time,
        ped_arrival_rate: f32,
        veh_arrival_rate: f32,
        road: Road) -> EventDrivenSim {

        assert!(end_time > start_time);
        assert!(ped_arrival_rate >= 0.0);
        assert!(veh_arrival_rate >= 0.0);

        // Set the random seed.
        // See https://stackoverflow.com/questions/59020767/how-can-i-input-an-integer-seed-for-producing-random-numbers-using-the-rand-crat
        let mut rng = StdRng::seed_from_u64(seed);

        // Generate pedestrian & vehicle arrival times.
        let ped_arrival_times = arrival_times(&start_time, &end_time, ped_arrival_rate, &mut rng);
        let veh_arrival_times = arrival_times(&start_time, &end_time, veh_arrival_rate, &mut rng);

        // Construct initial (empty) state at time 0.
        let state = Box::new(SimulatorState::new());

        let sim = EventDrivenSim {
            seed,
            start_time,
            end_time,
            ped_arrival_rate,
            veh_arrival_rate,
            ped_arrival_times,
            ped_arrival_idx: 0,
            veh_arrival_times,
            veh_arrival_idx: 0,
            road,
            state
        };

        sim
    }

    // Set the state arbitrarily. Useful for testing, but private.
    fn set_state(&mut self, state: Box<dyn State>) {
        self.state = state;
    }

    fn set_ped_arrival_times(&mut self, ped_arrival_times: Vec<Time>) {
        self.ped_arrival_times = ped_arrival_times;
    }

    fn set_veh_arrival_times(&mut self, veh_arrival_times: Vec<Time>) {
        self.veh_arrival_times = veh_arrival_times;
    }

    // pub fn current_state() -> State {

    // }

    fn new_vehicle(&mut self) -> &dyn Vehicle { todo!() }
    fn new_pedestrian(&mut self) -> &dyn Person { todo!() }
    fn remove_vehicle(&mut self, vehicle: &dyn Vehicle) { todo!() }
    fn remove_pedestrian(&mut self, pedestrian: &dyn Person) { todo!() }

}

impl Simulation for EventDrivenSim {

    // get time interval until next event
    fn time_to_next_event(&self) -> TimeDelta {

        // get min of pedestrian and vehicle arrival times
        // let min_ped_times = self.ped_arrival_times.iter().min().unwrap();
        // let min_veh_times = self.veh_arrival_times.iter().min().unwrap();

        let curr_time = *self.state.timestamp();
        let mut events= vec![self.end_time];

        if let Some(&arrival_time) = self.ped_arrival_times.get(self.ped_arrival_idx+1) {
            events.push(arrival_time);
        }
        if let Some(&arrival_time) = self.veh_arrival_times.get(self.veh_arrival_idx+1) {
            events.push(arrival_time);
        }

        let curr_vehicles = self.state.get_vehicles();
        for vehicle in curr_vehicles {
            let accel = vehicle.get_acceleration();
            if accel > 0.0 {
                let speed_delta = vehicle::MAX_SPEED - vehicle.get_speed();
                let t_delta = TimeDelta::from(speed_delta / accel);
                events.push(curr_time + t_delta);
            } else if accel < 0.0 {
                let t_delta = TimeDelta::from(vehicle.get_speed() / -vehicle.get_acceleration());
                events.push(curr_time + t_delta);
            }

            // Logic to check for obstacle events

        }

        // This is infallible since the vector always contains the termination time
        let min_time = events.iter().min().unwrap();
        TimeDelta::from(min_time - curr_time)
    }

    // roll state forward by time interval
    fn roll_forward_by(&mut self, time_delta: TimeDelta) {

    }

    // update state
    fn instantaneous_update(&mut self) {

    }

    fn handle_event(&mut self, event: Event<'_>) -> EventResult<'_> {
        use EventType::*;
        match event.1 {
            VehicleArrival => {
                EventResult::NewVehicle(self.new_vehicle())
            }
            VehicleExit(vehicle) => {
                self.remove_vehicle(vehicle);
                EventResult::RemoveVehicle
            }
            SpeedLimitReached(vehicle) => {
                vehicle.action(Action::StaticSpeed);
                EventResult::VehicleChange(vehicle)
            }
            ZeroSpeedReached(vehicle) => {
                vehicle.action(Action::StaticSpeed);
                EventResult::VehicleChange(vehicle)
            }
            ReactionToObstacle(vehicle) => {
                EventResult::VehicleChange(vehicle)
            }
            PedestrianArrival => {
                EventResult::NewPedestrian(self.new_pedestrian())
            }
            PedestrianExit(person) => {
                EventResult::RemovePedestrian
            }
            LightsToRed(crossing) => {
                EventResult::CrossingChange(crossing)
            }
            LightsToGreen(crossing) => {
                EventResult::CrossingChange(crossing)
            }
            StopSimulation => {

            }
            _ => unreachable!()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn test_sim() -> EventDrivenSim {
        let road = Road::new(100.0, Vec::new());
        EventDrivenSim::new(147, 0, 60000, 0.1, 0.2, road)
    }

    #[test]
    fn test_set_state() {

        let mut sim = test_sim();
        assert_eq!(sim.state.timestamp(), &0);

        // Construct a new state with a non-zero timestamp.
        let new_timestamp = 10000;
        let new_state = SimulatorState::dummy(Vec::new(), Vec::new(), new_timestamp);

        // Set the simulation state.
        sim.set_state(Box::new(new_state));
        assert_eq!(sim.state.timestamp(), &new_timestamp);
    }

    #[test]
    fn test_set_arrival_times() {

        let mut sim = test_sim();
        assert_ne!(sim.ped_arrival_times, vec!(10000, 20000));

        // Construct new pedestrian arrival times.
        let ped_arrival_times = vec!(10000, 20000);

        // Set the simulation pedestrian arrival times.
        sim.set_ped_arrival_times(ped_arrival_times);
        assert_eq!(sim.ped_arrival_times, vec!(10000, 20000));

        // Construct new vehicle arrival times.
        let veh_arrival_times = vec!(12000, 21000);

        // Set the simulation vehicle arrival times.
        sim.set_veh_arrival_times(veh_arrival_times);
        assert_eq!(sim.veh_arrival_times, vec!(12000, 21000));
    }

    #[test]
    fn test_pedestrian_arrival_event() {

        let mut sim = test_sim();

        let ped_arrival_times = vec!(10000, 20000);
        let veh_arrival_times = vec!(12000, 21000);

        // Set the pedestrian & vehicle arrival times.
        sim.set_ped_arrival_times(ped_arrival_times);
        sim.set_veh_arrival_times(veh_arrival_times);

        let actual = sim.time_to_next_event();
        assert_eq!(actual, TimeDelta::new(10000));
    }

    #[test]
    fn test_vehicle_arrival_event() {

        let mut sim = test_sim();

        let ped_arrival_times = vec!(5000, 7000);
        let veh_arrival_times = vec!(4000, 15000);

        // Set the pedestrian & vehicle arrival times.
        sim.set_ped_arrival_times(ped_arrival_times);
        sim.set_veh_arrival_times(veh_arrival_times);

        let actual = sim.time_to_next_event();
        assert_eq!(actual, TimeDelta::new(4000));
    }

    #[test]
    fn test_vehicle_stopping_event() {

        // let vehicles = vec!(Car::new(0.0));

        // TODO NEXT.
        // let state = SimulatorState::dummy();

    }

    //
    // Test the static helper functions.
    //
}
