use rand::{SeedableRng}; // SeedableRng needed for the seed_from_u64 method.
use rand::rngs::StdRng;
use rand::distributions::WeightedIndex;
use rand::prelude::Distribution;
use serde_json::to_string as to_json;

use crate::events::{Event, EventResult, EventType};
use crate::pedestrian::Person;
use crate::{ID, Time, pedestrian};
use crate::pedestrian::Pedestrian;
use crate::time::{TimeDelta, TIME_RESOLUTION};
use crate::simulation::{Simulation, arrival_times};
use crate::vehicle::{self, Action, Vehicle, Car, ACCELERATION_VALUE, DECCELERATION_VALUE, MAX_SPEED};
use crate::road::{Road, Direction, Crossing};
use crate::state::{State, SimulatorState};
use crate::obstacle::Obstacle;

pub struct EventDrivenSim {

    seed: u64,
    rng: StdRng,

    start_time: Time,
    end_time: Time,

    ped_arrival_rate: f32,
    veh_arrival_rate: f32,

    pub ped_arrival_times: Vec<Time>,
    pub veh_arrival_times: Vec<Time>,

    ped_counter: ID,
    veh_counter: ID,
    // dist: WeightedIndex<T>,

    road: Road,
    pub state: Box<dyn State>
}

impl EventDrivenSim {

    pub fn new(
	seed: u64,
        start_time: Time,
        end_time: Time,
        ped_arrival_rate: f32,
        veh_arrival_rate: f32,
	    // crossing_weights: Vec<f64>,
        road: Road) -> EventDrivenSim {

        assert!(end_time > start_time);
        assert!(ped_arrival_rate >= 0.0);
        assert!(veh_arrival_rate >= 0.0);

        // Set the random seed.
        // See https://stackoverflow.com/questions/59020767/how-can-i-input-an-integer-seed-for-producing-random-numbers-using-the-rand-crat
        let mut rng = StdRng::seed_from_u64(seed);

    	// let dist = WeightedIndex::new(&crossing_weights).unwrap();

        // Generate pedestrian & vehicle arrival times.
        let ped_arrival_times = arrival_times(&start_time, &end_time, ped_arrival_rate, &mut rng);
        let veh_arrival_times = arrival_times(&start_time, &end_time, veh_arrival_rate, &mut rng);

        // TODO: Make vector of pedestrian and vehicle ids
        // let pedestrians = generate_pedestrians();
        // let cars = generate_pedestrians();
        // vec: 0..veh_arrival_times.len()

        // Construct initial (empty) state at time 0.
        let state = Box::new(SimulatorState::new());

        let sim = EventDrivenSim {
            seed,
            rng,
            start_time,
            end_time,
            ped_arrival_rate,
            veh_arrival_rate,
            ped_arrival_times,
            veh_arrival_times,
            ped_counter: 0,
            veh_counter: 0,
            // dist,
            road,
            state
        };

        sim
    }

    // Set the state arbitrarily. Useful for testing, but private.
    fn set_state(&mut self, state: Box<dyn State>) {
        self.state = state;
    }

    pub fn set_ped_arrival_times(&mut self, ped_arrival_times: Vec<Time>) {
        self.ped_arrival_times = ped_arrival_times;
    }

    pub fn set_veh_arrival_times(&mut self, veh_arrival_times: Vec<Time>) {
        self.veh_arrival_times = veh_arrival_times;
    }

    fn generate_ped(&mut self) {
	// self.state.add_ped();
	self.ped_counter += 1;
    }

    fn generate_veh(&mut self) {
	// self.state.add_veh();
	self.veh_counter += 1;
    }
    // pub fn current_state() -> State {

    // }

    fn new_vehicle(&mut self) -> &dyn Vehicle {
        let direction_dist = rand::distributions::WeightedIndex::new(&[0.5, 0.5]).unwrap();
        let direction = if direction_dist.sample(&mut self.rng) == 0{
            Direction::Up
        } else {
            Direction::Down
        };

        let vehicle = Car::new(0, direction, MAX_SPEED, Action::StaticSpeed);
        let idx = self.state.push_vehicle(Box::new(vehicle));
        self.state.get_vehicle(idx)
    }

    fn new_pedestrian(&mut self) -> &dyn Person {
        let n_crossings = self.road.get_crossings(&Direction::Up).len();
        let idx_dist = rand::distributions::WeightedIndex::new(vec![1./n_crossings as f32; n_crossings]).unwrap();
        let (ref crossing, _) = self.road.get_crossings(&Direction::Up)[idx_dist.sample(&mut self.rng)];

        let id = self.ped_counter;
        self.ped_counter += 1;

        let pedestrian = Pedestrian::new(id, crossing, *self.state.timestamp());
        let idx =self.state.push_pedestrian(pedestrian);
        self.state.get_pedestrian(idx)
    }

    fn remove_vehicle(&mut self, idx: usize) {
        self.state.pop_vehicle(idx);
    }

    fn remove_pedestrian(&mut self, idx: usize) {
        self.state.pop_pedestrian(idx);
    }

    fn time_to_obstacle_event<T:Obstacle + ?Sized>(&self, vehicle: &dyn Vehicle, obstacle: &dyn Obstacle) -> Option<f32> {

        let rel_accel = vehicle.relative_acceleration(obstacle);
        let rel_speed = vehicle.relative_speed(obstacle);
        let rel_position = vehicle.relative_position(obstacle, &self.get_road());
        let buffer_zone: f32 = vehicle.get_buffer_zone();

        if rel_accel < 0.0 {
            // Obstacle is accelerating away from the vehicle.
            None

        } else if rel_accel == 0.0 {

            // Obstacle is receding and we're not relatively accelerating.
            if rel_speed <= 0.0 {
                None
            } else{
                // We are at max speed, what time will we be in the braking zone
                Some((rel_speed - f32::sqrt(rel_speed*rel_speed - 2.0 * DECCELERATION_VALUE * (rel_position - buffer_zone))) / DECCELERATION_VALUE)
            }


        } else if rel_accel > 0.0 {
            // We are accelerating, what time will we be in the braking zone
            Some((rel_speed + f32::sqrt(rel_speed + 2.0 * rel_accel * (rel_position - buffer_zone))) / rel_accel)

        } else {unreachable!()}

    }
}

impl Simulation for EventDrivenSim {
    // get time interval until next event
    fn next_event(&mut self) -> Event {

        // Simulation finished event.
        let curr_time = *self.state.timestamp();
        let mut events= vec![Event(self.end_time, EventType::StopSimulation)];

        // Pedestrian arrival events.
        if let Some(&arrival_time) = self.ped_arrival_times.get((self.ped_counter) as usize) {
            if arrival_time > curr_time {
                events.push(Event(arrival_time, EventType::PedestrianArrival));
            }
        }

        // Vehicle arrival events
        if let Some(&arrival_time) = self.veh_arrival_times.get((self.veh_counter) as usize) {
            if arrival_time > curr_time {
                events.push(Event(arrival_time, EventType::VehicleArrival));
            }
        }

        // Vehicle reaching speed limit or zero speed events.
        let curr_vehicles = self.state.get_vehicles();
        for (i, vehicle) in curr_vehicles.iter().enumerate() {
            let accel = vehicle.get_acceleration();
            if accel > 0.0 {
                let speed_delta = vehicle::MAX_SPEED - vehicle.get_speed();
                let t_delta = TimeDelta::from(speed_delta / accel);
                events.push(Event(curr_time + t_delta, EventType::SpeedLimitReached(i)));
            } else if accel < 0.0 {
                let t_delta = TimeDelta::from(vehicle.get_speed() / -vehicle.get_acceleration());
                events.push(Event(curr_time + t_delta, EventType::ZeroSpeedReached(i)));
            }

            // Logic to check for obstacle-related events.

            // Crossing obstacles:
            if let Some((ref crossing_obstacle, _)) = vehicle.next_crossing(&self.get_road()) {
                if let Some(t_delta_crossing) = self.time_to_obstacle_event::<dyn Obstacle>(&**vehicle, *crossing_obstacle) {
                    let t_delta = TimeDelta::from(t_delta_crossing);
                    events.push(Event(curr_time + t_delta, EventType::ReactionToObstacle(i)));
                }
            }

            // Vehicle obstacles:
            if let Some(ref vehicle_obstacle) = vehicle.next_vehicle(curr_vehicles) {
                // Upcast vehicle_obstacle to the Base trait Obstacle.
                let obstacle: &dyn Obstacle = vehicle_obstacle.as_osbstacle();
                if let Some(t_delta_vehicle) = self.time_to_obstacle_event::<dyn Obstacle>(&**vehicle, obstacle){
                    let t_delta = TimeDelta::from(t_delta_vehicle);
                    events.push(Event(curr_time + t_delta, EventType::ReactionToObstacle(i)));
                }
            }
        }

        // This is infallible since the vector always contains the termination time
        events.into_iter().min().unwrap()
    }

    // roll state forward by time interval
    fn roll_forward_by(&mut self, time_delta: TimeDelta) {
        self.state.update(time_delta);
    }

    // Update the state instantaneously based on the type of event.
    fn instantaneous_update(&mut self, event_type: EventType) {

        use EventType::*;
        match event_type {
            VehicleArrival => {
                self.new_vehicle();
                // EventResult::NewVehicle(self.new_vehicle())
            }
            VehicleExit(idx) => {
                self.remove_vehicle(idx);
                // EventResult::RemoveVehicle
            }
            SpeedLimitReached(idx) => {
                let vehicle = self.state.get_mut_vehicle(idx);
                vehicle.action(Action::StaticSpeed);
                // EventResult::VehicleChange(&*vehicle)
            }
            ZeroSpeedReached(idx) => {
                let vehicle = self.state.get_mut_vehicle(idx);
                vehicle.action(Action::StaticSpeed);
                // EventResult::VehicleChange(&*vehicle)
            }
            ReactionToObstacle(idx) => {
                let vehicle = self.state.get_mut_vehicle(idx);
                vehicle.action(Action::Deccelerate);
                // EventResult::VehicleChange(&*vehicle)
            }
            PedestrianArrival => {
                // EventResult::NewPedestrian(self.new_pedestrian())
                self.new_pedestrian();
            }
            PedestrianExit(idx) => {
                self.remove_pedestrian(idx);
                // EventResult::RemovePedestrian
            }
            LightsToRed(idx) => {
                let (crossing, _) = &self.road.get_crossings(&Direction::Up)[idx];
                // TODO.
                // EventResult::CrossingChange(crossing)
            }
            LightsToGreen(idx) => {
                let (crossing, _) = &self.road.get_crossings(&Direction::Up)[idx];
                // TODO.
                // EventResult::CrossingChange(crossing)
            }
            StopSimulation => {
                // EventResult::NoEffect
                // Nothing to do.
            }
            _ => unreachable!()
        }
    }

    fn get_state(&self) -> &Box<dyn State> {
        &self.state
    }

    // Moved to instantaneous_update (EventResult appears to be superfluous):
    // fn handle_event(&mut self, event: Event) -> EventResult<'_> {
    //     use EventType::*;
    //     match event.1 {
    //         VehicleArrival => {
    //             EventResult::NewVehicle(self.new_vehicle())
    //         }
    //         VehicleExit(idx) => {
    //             self.remove_vehicle(idx);
    //             EventResult::RemoveVehicle
    //         }
    //         SpeedLimitReached(idx) => {
    //             let vehicle = self.state.get_mut_vehicle(idx);
    //             vehicle.action(Action::StaticSpeed);
    //             EventResult::VehicleChange(&*vehicle)
    //         }
    //         ZeroSpeedReached(idx) => {
    //             let vehicle = self.state.get_mut_vehicle(idx);
    //             vehicle.action(Action::StaticSpeed);
    //             EventResult::VehicleChange(&*vehicle)
    //         }
    //         ReactionToObstacle(idx) => {
    //             let vehicle = self.state.get_mut_vehicle(idx);

    //             EventResult::VehicleChange(&*vehicle)
    //         }
    //         PedestrianArrival => {
    //             EventResult::NewPedestrian(self.new_pedestrian())
    //         }
    //         PedestrianExit(idx) => {
    //             self.remove_pedestrian(idx);
    //             EventResult::RemovePedestrian
    //         }
    //         LightsToRed(idx) => {
    //             let (crossing, _) = &self.road.get_crossings(&Direction::Up)[idx];
    //             EventResult::CrossingChange(crossing)
    //         }
    //         LightsToGreen(idx) => {
    //             let (crossing, _) = &self.road.get_crossings(&Direction::Up)[idx];
    //             EventResult::CrossingChange(crossing)
    //         }
    //         StopSimulation => {
    //             EventResult::NoEffect
    //         }
    //         _ => unreachable!()
    //     }
    // }

    fn get_road(&self) -> &Road {
        &self.road
    }

    // TODO. Move this to the Simulation trait.
    // Generic event-driven simulation algorithm.
    fn run(&mut self) -> () {

        let mut t: Time = 0;
        while t < self.end_time {

            let next_event = self.next_event();

            self.roll_forward_by(TimeDelta::new(next_event.0 - t));

            self.instantaneous_update(next_event.1);

            t = next_event.0;

            // Temp:
            let as_json= to_json(self.get_state()).unwrap();
            println!("{}", &as_json);
        }
    }
}


#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use crate::vehicle::{DECCELERATION_VALUE, MAX_SPEED};
    use super::*;

    fn dummy_sim() -> EventDrivenSim {
        let road = Road::new(100.0, Vec::new());
        EventDrivenSim::new(147, 0, 500_000, 0.1, 0.2, road)
    }

    fn dummy_no_arrivals_sim() -> EventDrivenSim {
        let road = Road::new(100.0, Vec::new());
        EventDrivenSim::new(147, 0, 500_000, 0.0, 0.0, road)
    }

    #[test]
    fn test_set_state() {

        let mut sim = dummy_sim();
        assert_eq!(sim.state.timestamp(), &0);

        // Construct a new state with a non-zero timestamp.
        let new_timestamp = 10000;
        let new_state = SimulatorState::dummy(VecDeque::new(), VecDeque::new(), new_timestamp);

        // Set the simulation state.
        sim.set_state(Box::new(new_state));
        assert_eq!(sim.state.timestamp(), &new_timestamp);
    }

    #[test]
    fn test_set_arrival_times() {

        let mut sim = dummy_sim();
        assert_ne!(sim.ped_arrival_times, vec!(10000, 20000));

        // Construct new pedestrian arrival times.
        let ped_arrival_times = vec!(10000, 20000);

        // Set the simulation pedestrian arrival times.
        sim.set_ped_arrival_times(ped_arrival_times);
        assert_eq!(sim.ped_arrival_times, vec!(10000, 20000));

        // Construct new vehicle arrival times.
        let veh_arrival_times = vec!(12000, 21000);

        // Set the simulation vehicle arrival times.Car::new(0u64, Direction::Up, speed, Action::Deccelerate))
        sim.set_veh_arrival_times(veh_arrival_times);
        assert_eq!(sim.veh_arrival_times, vec!(12000, 21000));
    }

    #[test]
    fn test_pedestrian_arrival_event() {

        let mut sim = dummy_sim();

        let ped_arrival_times = vec!(10000, 20000);
        let veh_arrival_times = vec!(12000, 21000);

        // Set the pedestrian & vehicle arrival times.
        sim.set_ped_arrival_times(ped_arrival_times);
        sim.set_veh_arrival_times(veh_arrival_times);

        let actual = sim.next_event();
        assert_eq!(actual.0, 10000);
    }

    #[test]
    fn test_vehicle_arrival_event() {

        let mut sim = dummy_sim();

        let ped_arrival_times = vec!(5000, 7000);
        let veh_arrival_times = vec!(4000, 15000);

        // Set the pedestrian & vehicle arrival times.
        sim.set_ped_arrival_times(ped_arrival_times);
        sim.set_veh_arrival_times(veh_arrival_times);

        let actual = sim.next_event();
        assert_eq!(actual.0, 4000);
    }

    #[test]
    fn test_vehicle_stopping_event() {

        let speed = 10.0;
        let mut vehicles: VecDeque<Box<dyn Vehicle>> = VecDeque::new();
        vehicles.push_back(Box::new(Car::new(0u64, Direction::Up, speed, Action::Deccelerate)));


        let timestamp = 22 * TIME_RESOLUTION;
        let state = SimulatorState::dummy(vehicles, VecDeque::new(), timestamp);

        let mut sim = dummy_sim();
        sim.set_state(Box::new(state));

        let actual= sim.next_event();
        assert_eq!(actual.0, timestamp + TimeDelta::from((-1.0) * (speed / DECCELERATION_VALUE)));
    }

    #[test]
    fn test_vehicle_speed_limit_event() {

        let speed = 10.0;
        let vehicles: Vec<Box<dyn Vehicle>> = vec!(Box::new(Car::new(0u64, Direction::Up, speed, Action::Accelerate)));

        let timestamp = 11 * TIME_RESOLUTION;
        let state = SimulatorState::dummy(vehicles.into(), VecDeque::new(), timestamp);

        let mut sim = dummy_sim();
        sim.set_state(Box::new(state));

        let actual = sim.next_event();
        assert_eq!(actual.0, timestamp + TimeDelta::from((MAX_SPEED - speed) / ACCELERATION_VALUE));
    }
}
    // TODO: uncomment new tests below based on config when ready
//     #[test]
//     fn test_vehicle_reaction_event() {

//         // TODO.
//         // Use the "dummy" Car constructor to make two cars with given initial positions.
//         let v1 = Car::new(0,Direction::Up, 0.0, Action::StaticSpeed);
//         let v2 = Car::new(1, Direction::Up, MAX_SPEED, Action::StaticSpeed);
//         let vehicles: Vec<Box<dyn Vehicle>> = vec!(Box::new(v1), Box::new(v2));
//         let timestamp = 24 * TIME_RESOLUTION;
//         let state = SimulatorState::dummy(vehicles.into(), VecDeque::new(), timestamp);

//         let mut sim = dummy_no_arrivals_sim();
//         sim.set_state(Box::new(state));


// #[cfg(test)]
// mod tests {
//     use std::collections::VecDeque;
//     use crate::vehicle::{DECCELERATION_VALUE, MAX_SPEED};
//     use super::*;

// // <<<<<<< HEAD
// //     fn test_sim() -> EventDrivenSim {
// //         let road = Road::new();
// //         EventDrivenSim::new(147, 0, 60000, 0.1, 0.2, road)
// // ||||||| 98a1048
// //     fn test_sim() -> EventDrivenSim {
// //         let road = Road::new(100.0, Vec::new());
// //         EventDrivenSim::new(147, 0, 60000, 0.1, 0.2, road)
// // =======
//     fn dummy_sim() -> EventDrivenSim {
//         let road = Road::new();
//         EventDrivenSim::new(147, 0, 500_000, 0.1, 0.2, road)
//     }

//     fn dummy_no_arrivals_sim() -> EventDrivenSim {
//         let road = Road::new();
//         EventDrivenSim::new(147, 0, 500_000, 0.0, 0.0, road)
// // >>>>>>> main
//     }

//     #[test]
//     fn test_set_state() {

//         let mut sim = dummy_sim();
//         assert_eq!(sim.state.timestamp(), &0);

//         // Construct a new state with a non-zero timestamp.
//         let new_timestamp = 10000;
//         let new_state = SimulatorState::dummy(VecDeque::new(), VecDeque::new(), new_timestamp);

//         // Set the simulation state.
//         sim.set_state(Box::new(new_state));
//         assert_eq!(sim.state.timestamp(), &new_timestamp);
//     }

//     #[test]
//     fn test_set_arrival_times() {

//         let mut sim = dummy_sim();
//         assert_ne!(sim.ped_arrival_times, vec!(10000, 20000));

//         // Construct new pedestrian arrival times.
//         let ped_arrival_times = vec!(10000, 20000);

//         // Set the simulation pedestrian arrival times.
//         sim.set_ped_arrival_times(ped_arrival_times);
//         assert_eq!(sim.ped_arrival_times, vec!(10000, 20000));

//         // Construct new vehicle arrival times.
//         let veh_arrival_times = vec!(12000, 21000);

//         // Set the simulation vehicle arrival times.Car::new(0u64, Direction::Up, speed, Action::Deccelerate))
//         sim.set_veh_arrival_times(veh_arrival_times);
//         assert_eq!(sim.veh_arrival_times, vec!(12000, 21000));
//     }

//     #[test]
//     fn test_pedestrian_arrival_event() {

//         let mut sim = dummy_sim();

//         let ped_arrival_times = vec!(10000, 20000);
//         let veh_arrival_times = vec!(12000, 21000);

//         // Set the pedestrian & vehicle arrival times.
//         sim.set_ped_arrival_times(ped_arrival_times);
//         sim.set_veh_arrival_times(veh_arrival_times);

//         let actual = sim.next_event();
//         assert_eq!(actual.0, 10000);
//     }

//     #[test]
//     fn test_vehicle_arrival_event() {

//         let mut sim = dummy_sim();

//         let ped_arrival_times = vec!(5000, 7000);
//         let veh_arrival_times = vec!(4000, 15000);

//         // Set the pedestrian & vehicle arrival times.
//         sim.set_ped_arrival_times(ped_arrival_times);
//         sim.set_veh_arrival_times(veh_arrival_times);

//         let actual = sim.next_event();
//         assert_eq!(actual.0, 4000);
//     }

//     #[test]
//     fn test_vehicle_stopping_event() {

//         let speed = 10.0;
//         let mut vehicles: VecDeque<Box<dyn Vehicle>> = VecDeque::new();
//         vehicles.push_back(Box::new(Car::new(0u64, Direction::Up, speed, Action::Deccelerate)));


//         let timestamp = 22 * TIME_RESOLUTION;
//         let state = SimulatorState::dummy(vehicles, VecDeque::new(), timestamp);

//         let mut sim = dummy_sim();
//         sim.set_state(Box::new(state));

//         let actual= sim.next_event();
//         assert_eq!(actual.0, timestamp + TimeDelta::from((-1.0) * (speed / DECCELERATION_VALUE)));
//     }

//     #[test]
//     fn test_vehicle_speed_limit_event() {

//         let speed = 10.0;
//         let vehicles: Vec<Box<dyn Vehicle>> = vec!(Box::new(Car::new(0u64, Direction::Up, speed, Action::Accelerate)));

//         let timestamp = 11 * TIME_RESOLUTION;
//         let state = SimulatorState::dummy(vehicles.into(), VecDeque::new(), timestamp);

//         let mut sim = dummy_sim();
//         sim.set_state(Box::new(state));

//         let actual = sim.next_event();
//         assert_eq!(actual.0, timestamp + TimeDelta::from((MAX_SPEED - speed) / ACCELERATION_VALUE));
//     }

//     #[test]
//     fn test_vehicle_reaction_event() {

//         // TODO.
//         // Use the "dummy" Car constructor to make two cars with given initial positions.
//         let v1 = Car::new(0,Direction::Up, 0.0, Action::StaticSpeed);
//         let v2 = Car::new(1, Direction::Up, MAX_SPEED, Action::StaticSpeed);
//         let vehicles: Vec<Box<dyn Vehicle>> = vec!(Box::new(v1), Box::new(v2));
//         let timestamp = 24 * TIME_RESOLUTION;
//         let state = SimulatorState::dummy(vehicles.into(), VecDeque::new(), timestamp);

//         let mut sim = dummy_no_arrivals_sim();
//         sim.set_state(Box::new(state));

//         // TODO.
//         // Compute the time delta before the trailing car (v2) reaches the Brake region.


//     }
//     //
//     // Test the static helper functions.
//     //
// }
