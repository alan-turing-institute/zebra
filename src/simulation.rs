
use rand_distr::{Exp, Distribution};
use rand::{SeedableRng}; // SeedableRng needed for the seed_from_u64 method.
use rand::rngs::StdRng;
use crate::events::{Event, EventResult, EventType};

use crate::Time;
use crate::road::Road;
use crate::time::TIME_RESOLUTION;
use crate::time::TimeDelta;
use crate::state::{State, SimulatorState};
use crate::vehicle::{Vehicle, Car};


pub trait Simulation {
    // get time interval until next event
    fn next_event(&mut self) -> Event;

    // roll simulation forward by time interval
    fn roll_forward_by(&mut self, time_delta: TimeDelta);

    // update simulation state
    fn instantaneous_update(&mut self);

    fn get_state(&self) -> &Box<dyn State> ;

    fn handle_event(&mut self, event: Event) -> EventResult<'_>;
    // {
    //     use EventType::*;
    //     match event.1 {
    //         VehicleArrival => {}
    //         VehicleExit(vehicle) => {}
    //         SpeedLimitReached(_) => {}
    //         ZeroSpeedReached(_) => {}
    //         ReactionToObstacle(_) => {}
    //         PedestrianArrival => {}
    //         PedestrianExit(_) => {}
    //         LightsToRed(_) => {}
    //         LightsToGreen(_) => {}
    //         _ => unreachable!()
    //     }
    // }
    fn get_road(&self) -> &Road;
}

// MOVED TO EventDrivenSim
// pub struct Simulation {

//     seed: u64,

//     start_time: Time,
//     end_time: Time,

//     ped_arrival_rate: f32,
//     veh_arrival_rate: f32,

//     pub ped_arrival_times: Vec<Time>,
//     pub veh_arrival_times: Vec<Time>,

//     road: Road,
//     state: Box<dyn State>
// }

// impl Simulation {

//     pub fn new(seed: u64,
//         start_time: Time,
//         end_time: Time,
//         ped_arrival_rate: f32,
//         veh_arrival_rate: f32,
//         road: Road) -> Simulation {

//         assert!(end_time > start_time);
//         assert!(ped_arrival_rate >= 0.0);
//         assert!(veh_arrival_rate >= 0.0);

//         // Set the random seed.
//         // See https://stackoverflow.com/questions/59020767/how-can-i-input-an-integer-seed-for-producing-random-numbers-using-the-rand-crat
//         let mut rng = StdRng::seed_from_u64(seed);

//         // Generate pedestrian & vehicle arrival times.
//         let ped_arrival_times = arrival_times(&start_time, &end_time, ped_arrival_rate, &mut rng);
//         let veh_arrival_times = arrival_times(&start_time, &end_time, veh_arrival_rate, &mut rng);

//         // Construct initial (empty) state at time 0.
//         let state = Box::new(SimulatorState::new());

//         let sim = Simulation {
//             seed,
//             start_time,
//             end_time,
//             ped_arrival_rate,
//             veh_arrival_rate,
//             ped_arrival_times,
//             veh_arrival_times,
//             road,
//             state
//         };

//         sim
//     }

//     // Set the state arbitrarily. Useful for testing, but private.
//     fn set_state(&mut self, state: Box<dyn State>) {
//         self.state = state;
//     }

//     fn set_ped_arrival_times(&mut self, ped_arrival_times: Vec<Time>) {
//         self.ped_arrival_times = ped_arrival_times;
//     }

//     fn set_veh_arrival_times(&mut self, veh_arrival_times: Vec<Time>) {
//         self.veh_arrival_times = veh_arrival_times;
//     }

//     // pub fn current_state() -> State {

//     // }
// }

// impl SimTrait for Simulation {

//     // get time interval until next event
//     fn time_to_next_event(&self) -> TimeDelta {

//         // get min of pedestrian and vehicle arrival times
//         let min_ped_times = self.ped_arrival_times.iter().min().unwrap();
//         let min_veh_times = self.veh_arrival_times.iter().min().unwrap();

//         // return the smallest of the two times as the next event
//         if min_veh_times < min_ped_times {
//             return TimeDelta::new(*min_veh_times);
//         }
//         TimeDelta::new(*min_ped_times)
//     }

//     // roll state forward by time interval
//     fn roll_forward_by(&mut self, time_delta: TimeDelta) {

//     }
    // }


// }

//     // update state
//     fn instantaneous_update(&mut self) {

//     }
// }

pub fn arrival_times(start_time: &Time, end_time: &Time, arrival_rate: f32, rng: &mut StdRng) -> Vec<Time> {

    let mut ret = Vec::new();
    let mut t = start_time.clone();
    loop {
        t = t + interarrival_time(arrival_rate, rng);
        if &t > end_time { break ret }
        ret.push(t);
    }
}

pub fn interarrival_time(arrival_rate: f32, rng: &mut StdRng) -> Time {
    let exp = Exp::new(arrival_rate).unwrap(); // see https://docs.rs/rand_distr/0.2.1/rand_distr/struct.Exp.html
    f32::round(exp.sample(rng) * (TIME_RESOLUTION as f32)) as i64
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     fn test_sim() -> Simulation {
//         let road = Road::new(100.0, Vec::new());
//         Simulation::new(147, 0, 60000, 0.1, 0.2, road)
//     }

//     #[test]
//     fn test_set_state() {

//         let mut sim = test_sim();
//         assert_eq!(sim.state.timestamp(), &0);

//         // Construct a new state with a non-zero timestamp.
//         let new_timestamp = 10000;
//         let new_state = SimulatorState::dummy(Vec::new(), Vec::new(), new_timestamp);

//         // Set the simulation state.
//         sim.set_state(Box::new(new_state));
//         assert_eq!(sim.state.timestamp(), &new_timestamp);
//     }

//     #[test]
//     fn test_set_arrival_times() {

//         let mut sim = test_sim();
//         assert_ne!(sim.ped_arrival_times, vec!(10000, 20000));

//         // Construct new pedestrian arrival times.
//         let ped_arrival_times = vec!(10000, 20000);

//         // Set the simulation pedestrian arrival times.
//         sim.set_ped_arrival_times(ped_arrival_times);
//         assert_eq!(sim.ped_arrival_times, vec!(10000, 20000));

//         // Construct new vehicle arrival times.
//         let veh_arrival_times = vec!(12000, 21000);

//         // Set the simulation vehicle arrival times.
//         sim.set_veh_arrival_times(veh_arrival_times);
//         assert_eq!(sim.veh_arrival_times, vec!(12000, 21000));
//     }

//     #[test]
//     fn test_pedestrian_arrival_event() {

//         let mut sim = test_sim();

//         let ped_arrival_times = vec!(10000, 20000);
//         let veh_arrival_times = vec!(12000, 21000);

//         // Set the pedestrian & vehicle arrival times.
//         sim.set_ped_arrival_times(ped_arrival_times);
//         sim.set_veh_arrival_times(veh_arrival_times);

//         let actual = sim.time_to_next_event();
//         assert_eq!(actual, TimeDelta::new(10000));
//     }

//     #[test]
//     fn test_vehicle_arrival_event() {

//         let mut sim = test_sim();

//         let ped_arrival_times = vec!(5000, 7000);
//         let veh_arrival_times = vec!(4000, 15000);

//         // Set the pedestrian & vehicle arrival times.
//         sim.set_ped_arrival_times(ped_arrival_times);
//         sim.set_veh_arrival_times(veh_arrival_times);

//         let actual = sim.time_to_next_event();
//         assert_eq!(actual, TimeDelta::new(4000));
//     }

//     #[test]
//     fn test_vehicle_stopping_event() {

//         let vehicles = vec!(Car::new(0.0));

//         // TODO NEXT.
//         // let state = SimulatorState::dummy();

//     }

//     //
//     // Test the static helper functions.
//     //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interarrival_time() {

        // Set the random seed.
        let mut rng = StdRng::seed_from_u64(147);
        let actual = interarrival_time(2.0, &mut rng);
        let expected = 266;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_arrival_times() {

        // Set the random seed.
        let mut rng = StdRng::seed_from_u64(147);

        // With this seed, there are 2 arrivals in 10 seconds.
        let end_time = 10 * TIME_RESOLUTION;
        let actual = arrival_times(&0, &end_time, 0.2, &mut rng);
        assert_eq!(actual.len(), 2);

        // Check that all arrivals occur before the simulation end time.
        assert!(actual.iter().max().unwrap() <= &end_time);
    }
}
