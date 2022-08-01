use rand::{SeedableRng}; // SeedableRng needed for the seed_from_u64 method.
use rand::rngs::StdRng;
use rand::distributions::WeightedIndex;
use rand::prelude::Distribution;
use serde_json::to_string_pretty as to_json;
use serde_json::to_string as to_json_flat;

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
use std::rc::Rc;
use crate::{raw_input};
use std::fs::{OpenOptions};
use std::io::Write;

const THRESHOLD_REACT: f32 = -0.001;
const THRESHOLD_ACCELERATE: f32 = 1.;
const MIN_DIST_TO_OBS: f32 = 1.;
const THRESHOLD_REL_SPEED: f32 = -0.1;
const TIME_TO_EVENT_ROUNDING: f32 = 1000.0;

pub struct EventDrivenSim  {

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
    pub state: Box<dyn State >,
    road: Road,
    verbose: bool
}

impl  EventDrivenSim  {

    pub fn new(
	    seed: u64,
        start_time: Time,
        end_time: Time,
        ped_arrival_rate: f32,
        veh_arrival_rate: f32,
	    // crossing_weights: Vec<f64>,
        state: Box<dyn State>,
        road: Road,
        verbose: bool
    ) -> Self {

        assert!(end_time > start_time);
        assert!(ped_arrival_rate >= 0.0);
        assert!(veh_arrival_rate >= 0.0);

        // Set the random seed.
        // See https://stackoverflow.com/questions/59020767/how-can-i-input-an-integer-seed-for-producing-random-numbers-using-the-rand-crat
        let mut rng = StdRng::seed_from_u64(seed);

    	// let dist = WeightedIndex::new(&crossing_weights).unwrap();

        // Generate pedestrian & vehicle arrival times.
        let ped_arrival_times = arrival_times(&start_time, &end_time, ped_arrival_rate, &mut rng);
        let mut veh_arrival_times = arrival_times(&start_time, &end_time, veh_arrival_rate, &mut rng);

        // Ensure big enough gap to brake: 13.41m/s to 0. is 3.35, so round to 3400ms
        for i in 0..veh_arrival_times.len() { 
            if i > 0 && veh_arrival_times[i] - veh_arrival_times[i] < 3400 {
                veh_arrival_times[i] = veh_arrival_times[i-1] + 3400
            }
        }

        // Construct initial (empty) state at time 0.
        // let state = Box::new(SimulatorState::new());
        // let state = Box::new(SimulatorState::new());

        Self {
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
            state,
            verbose
        }
    }

    // Set the state arbitrarily. Useful for testing, but private.
    // fn set_state<'b>(&'b mut self, state: Box<dyn State>) {
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
        // let direction_dist = rand::distributions::WeightedIndex::new(&[0.5, 0.5]).unwrap();
        let direction_dist = rand::distributions::WeightedIndex::new(&[1., 0.]).unwrap();
        let direction = if direction_dist.sample(&mut self.rng) == 0{
            Direction::Up
        } else {
            Direction::Down
        };

        let vehicle = Car::new(self.veh_counter, direction, MAX_SPEED, Action::StaticSpeed);
        // let vehicle = Car::new(self.veh_counter, direction, 0.0, Action::Accelerate);

        // Increment veh counter
        self.veh_counter += 1;

        let idx = self.state.push_vehicle(Box::new(vehicle));
        self.state.get_vehicle(idx)
    }
    // fn new_pedestrian(&'a mut self) -> &dyn Person {
    // fn new_pedestrian<'b>(&'b mut self) -> &dyn Person {
    fn new_pedestrian(&mut self) -> &dyn Person {
        let n_crossings = self.road.get_crossings(&Direction::Up).len();
        let idx_dist = rand::distributions::WeightedIndex::new(vec![1./n_crossings as f32; n_crossings]).unwrap();
        let (crossing, _) = &self.road.get_crossings(&Direction::Up)[idx_dist.sample(&mut self.rng)];

        let id = self.ped_counter;
        self.ped_counter += 1;

        let pedestrian = Pedestrian::new(id, Rc::clone(crossing), *self.state.timestamp());
        let idx =self.state.push_pedestrian(pedestrian);
        self.state.get_pedestrian(idx)
    }

    fn remove_vehicle(&mut self, idx: usize) {
        // If all vehicles are Up, then this should hold.
        // assert_eq!(idx, 0);
        self.state.pop_vehicle(idx);
    }

    fn remove_pedestrian(&mut self, id: ID) {
        for (idx, ped) in self.state.get_pedestrians().into_iter().enumerate() {
            if ped.get_id() == id {
                // It should always be oldest at front if crossing times are
                // the same for all pedestrians.
                // If this is correct, we can refactor this inefficient loop
                // and state to do `pedestrians.pop_front();`
                assert_eq!(idx, 0);
                self.state.pop_pedestrian(idx);
                break;
            }
        }
    }

    
    fn time_to_exit_event<T:Obstacle + ?Sized>(
        &self, vehicle: &dyn Vehicle,
        obstacle: &dyn Obstacle
    ) -> Option<f32> {
        let rel_accel = vehicle.relative_acceleration(obstacle);
        let rel_speed = vehicle.relative_speed(obstacle);
        let rel_position = vehicle.relative_position(obstacle, &self.get_road());

        assert!(rel_speed >= 0.0);

        if rel_speed == 0.0 {
            return None;
        }

        if rel_accel == 0.0 {
            return Some(-rel_position / rel_speed);
        } else if rel_accel > 0.0 {
            let t_delta = (
                -rel_speed + f32::sqrt(rel_speed * rel_speed - 2.0 * rel_position * rel_accel)
            )/rel_accel;

            return Some(t_delta);
        }
        else {
            if rel_speed * rel_speed > 2.0 * rel_position * rel_accel {
                let t_delta = (
                    -rel_speed + f32::sqrt(rel_speed * rel_speed - 2.0 * rel_position * rel_accel)
                )/rel_accel;
                return Some(t_delta);
            }
            return None;
        }
    }

    fn time_to_rel_speed_aim<T:Obstacle + ?Sized>(
        &self, vehicle: &dyn Vehicle,
        obstacle: &dyn Obstacle,
        rel_speed_aim: f32
    ) -> Option<f32> {
        let rel_accel = vehicle.relative_acceleration(obstacle);
        let rel_speed = vehicle.relative_speed(obstacle);

        assert!(rel_speed >= 0.0);
        assert!(rel_accel < 0.0);

        Some((rel_speed - rel_speed_aim)  / rel_accel)
    }

    fn time_to_obstacle_event<T:Obstacle + ?Sized>(
        &self, vehicle: &dyn Vehicle,
        obstacle: &dyn Obstacle,
        veh_acc: bool,
        obs_dec: bool
    ) -> Option<f32> {


        let mut rel_accel = vehicle.relative_acceleration(obstacle);
        let mut rel_speed = vehicle.relative_speed(obstacle);
        let mut rel_position = vehicle.relative_position(obstacle, &self.get_road());
        let buffer = vehicle.get_buffer_zone() + obstacle.get_obstacle_length();

        // If in standard react mode, and:
        //   vehicle already decelerating
        //   OR speed and accel less than or equal to obstacle
        // no time is returned
        if !veh_acc && (vehicle.get_acceleration() == DECCELERATION_VALUE || (rel_speed <= 0.0 && rel_accel <= 0.0)) {
            return None;
        }

        // If obs decelerating or in obs_dec mode
        if obstacle.get_acceleration() == DECCELERATION_VALUE || obs_dec {
            // Adjust relative values for future stopped obstacle
            let x2 = obstacle.get_position(&self.road, &vehicle.get_direction());
            let u2 = obstacle.get_speed();
            let a2 = DECCELERATION_VALUE;
            rel_position = rel_position + x2 - (x2 - ((u2 * u2) / (2.0 * a2)));
            rel_speed = rel_speed + u2;
            rel_accel = vehicle.get_acceleration();
        }

        // If switching to veh_acc mode
        if veh_acc {
            rel_accel = rel_accel - vehicle.get_acceleration() + ACCELERATION_VALUE;
        }

        // If already near buffer (arbitrary within 10%) and testing veh_acc switch,
        // do not acc so return time = 0
        if rel_position > -1.1 * buffer && veh_acc {
            return Some(0.0);
        }

        // Vehicle must be behind obstacle
        assert!(rel_position <= 0.0);

        // Gamma value for convenience
        let gamma = 1. - rel_accel / DECCELERATION_VALUE;
        
        // Case 1: rel_accel = 0
        if rel_accel == 0. {
            // If no relative speed
            if rel_speed <= 0.0 {
                return None;
            }
            let t_prime = (1. / rel_speed) * (-buffer + (rel_speed*rel_speed)/(2. * DECCELERATION_VALUE) - rel_position);
            return Some(f32::round(t_prime * TIME_TO_EVENT_ROUNDING)/TIME_TO_EVENT_ROUNDING);
        }
        // Case 2: rel_accel != 0
        else if rel_accel != 0.0 {
            let t_prime = (
                -rel_speed * gamma
                + f32::sqrt(
                    (rel_speed * gamma)*(rel_speed * gamma)
                    - 2. * rel_accel * gamma * (
                        rel_position - (rel_speed * rel_speed)/(2. * DECCELERATION_VALUE) + buffer
                    )
                )
            ) / (rel_accel * gamma);
            return Some(f32::round(t_prime * TIME_TO_EVENT_ROUNDING)/TIME_TO_EVENT_ROUNDING);
            
        } else {unreachable!()}
    }
}

impl  Simulation  for EventDrivenSim  {
    // get time interval until next event
    fn next_events(&mut self) -> Vec<Event> {

        // Simulation finished event.
        let curr_time = *self.state.timestamp();
        let mut events= vec![Event(self.end_time, EventType::StopSimulation)];

        // Pedestrian arrival events.
        if let Some(&arrival_time) = self.ped_arrival_times.get((self.ped_counter) as usize) {
            if arrival_time > curr_time {
                events.push(Event(arrival_time, EventType::PedestrianArrival));
            }
        }

        // Look over pedestrians to do exits
        for ped in self.state.get_pedestrians().into_iter() {
            events.push(Event(ped.location().stop_time() + ped.arrival_time(), EventType::PedestrianExit(ped.get_id())));
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
                let t_delta = TimeDelta::floor(speed_delta / accel);
                events.push(Event(curr_time + t_delta, EventType::SpeedLimitReached(i)));
            } else if accel < 0.0 && vehicle.get_speed() > 0.0 {
                let t_delta = TimeDelta::floor(vehicle.get_speed() / -vehicle.get_acceleration());
                events.push(Event(curr_time + t_delta, EventType::ZeroSpeedReached(i)));
            }

            // Logic to check for obstacle-related events.
            //
            // Exit time from treating as obstacle
            if let Some(exit_time) = self.time_to_exit_event::<dyn Obstacle>(&**vehicle, self.road.get_exit()) {
                let t_delta = TimeDelta::floor(exit_time);
                events.push(Event(curr_time + t_delta, EventType::VehicleExit(i)));
            }

            // Option for min reaction time across obstacles after a vehicles tries switching to accelerating
            let mut min_react_after_switch: Option<f32> = None;
            let mut min_dist_to_obs: Option<f32> = None;

            // Bool for no obstacles
            let mut no_ahead_obs = true;

            // Pedestrian obstacles:
            // Loop over pedestrians in state to get active pedestrians
            if let Some(obstacle) = vehicle.next_pedestrian(&self.get_road(), &self.state.get_pedestrians(), *self.state.timestamp())
            {
                // An obstacle is present
                no_ahead_obs = false;

                // Get time braking is required to stop in time for next pedestrian
                if let Some(t_delta) = self.time_to_obstacle_event::<dyn Obstacle>(&**vehicle, obstacle, false, false) {
                    // TODO: consider making t_delta, f32::max(0., t_delta) so always react even if too late.
                    if t_delta >= THRESHOLD_REACT {
                        // TODO: consider rounding issues in TimeDelta conversion
                        events.push(Event(curr_time + TimeDelta::floor(t_delta), EventType::ReactionToObstacle(i)));
                    }
                    else {
                        // If speed is non-zero, must emergency stop
                        if vehicle.get_speed() != 0.0 {
                            // Remove emergency stop for pedestrian as should never occur
                            // events.push(Event(curr_time, EventType::EmergencyStop(i)));
                        }
                    }
                }
                // If no reaction to next pedestrian, if vehicle starts accelerating, get reaction time
                // for braking to then begin in order to stop in time for pedestrian
                else if let Some(t_delta) = self.time_to_obstacle_event::<dyn Obstacle>(&**vehicle, obstacle, true, false) {
                    // Debugging
                    if self.verbose {
                        println!("Veh {} ped t_delta react after accel switch: {:?}", i, t_delta);
                    }
                    if min_react_after_switch == None {
                        min_react_after_switch = Some(t_delta);
                        min_dist_to_obs = Some(-vehicle.relative_position(obstacle, &self.road));
                    }
                }
            }

            // Vehicle obstacles:
            if let Some(ref vehicle_obstacle) = vehicle.next_vehicle(curr_vehicles) {
                // An obstacle is present
                no_ahead_obs = false;

                // Prevent vehicles from overtaking as shouldn't happen
                assert!(vehicle_obstacle.get_position(&self.road, &vehicle_obstacle.get_direction()) > vehicle.get_position(&self.road, &vehicle.get_direction()));
                assert!(vehicle_obstacle.get_id() < vehicle.get_id());

                if self.verbose {
                    println!("Vehicle: {}\nhas next Vehicle: {}\n", &to_json(vehicle).unwrap(), &to_json(vehicle_obstacle).unwrap());
                }

                // Upcast vehicle_obstacle to the Base trait Obstacle.
                let obstacle: &dyn Obstacle = vehicle_obstacle.as_obstacle();

                // Get time required to start braking if next vehicle immediately starts braking now
                if let Some(t_delta) = self.time_to_obstacle_event::<dyn Obstacle>(&**vehicle, obstacle, false, true) {
                    if t_delta >= THRESHOLD_REACT {
                        // TODO: consider rounding issues in TimeDelta conversion
                        events.push(Event(curr_time + TimeDelta::floor(t_delta), EventType::ReactionToObstacle(i)));
                    }
                    else {
                        // If speed is non-zero, must emergency stop
                        if vehicle.get_speed() != 0.0 {
                            // events.push(Event(curr_time, EventType::EmergencyStop(i)));
                        }
                    }
                }
                // If no reaction to next vehicle, if vehicle starts accelerating, get reaction time assuming
                // next vehicle immediately starts braking
                else if let Some(t_delta) = self.time_to_obstacle_event::<dyn Obstacle>(&**vehicle, obstacle, true, true) {
                    if min_react_after_switch == None {
                        min_react_after_switch = Some(t_delta);
                        // min_dist_to_obs = Some(-vehicle.relative_position(obstacle, &self.road));
                    } else {
                        min_react_after_switch = Some(f32::min(min_react_after_switch.unwrap(), t_delta));
                        // min_dist_to_obs = Some(f32::min(min_dist_to_obs.unwrap(), -vehicle.relative_position(obstacle, &self.road)));
                    }
                }

                // If decelerating and obstacle not, get time until relative speed is slightly negative (-0.01m/s)
                // and add event to switch to static speed ("follow") (providing no other events logged)
                if vehicle.get_acceleration() < 0.0 && !(obstacle.get_acceleration() < 0.0) && min_react_after_switch == None {
                    let mut rel_speed_aim = THRESHOLD_REL_SPEED;
                    if obstacle.get_speed() < -THRESHOLD_REL_SPEED {
                        rel_speed_aim = 0.0;
                    }
                    let t_delta = self.time_to_rel_speed_aim::<dyn Obstacle>(&**vehicle, obstacle, rel_speed_aim).unwrap();
                    events.push(Event(curr_time + TimeDelta::floor(t_delta), EventType::StaticSpeedReached(i)));
                }
            }

            // Debug
            if self.verbose {
                println!("Min react time for vehicle {} after switch: {:?}", i, min_react_after_switch);
            }

            // If switching to accelerate causes no immediate reaction AND not top speed, accelerate
            if vehicle.get_speed() < MAX_SPEED && vehicle.get_acceleration() != ACCELERATION_VALUE {
                if min_react_after_switch == None {
                    // If no obstacles are ahead, then accelerate
                    if no_ahead_obs {
                        events.push(Event(curr_time, EventType::VehicleAccelerate(i)));
                    }
                }
                else {
                    let t_delta = min_react_after_switch.unwrap();
                    let dist = min_dist_to_obs;
                    // Arbitrary time larger to ensure no looping between stop/start, choose 0.2s
                    if dist == None || dist.unwrap() > MIN_DIST_TO_OBS {
                        if t_delta > THRESHOLD_ACCELERATE {
                            events.push(Event(curr_time, EventType::VehicleAccelerate(i)));
                        }
                    }
                }
            }

        }

        // Print if verbose
        if self.verbose {
            // Sort events
            events.sort_by(|x, y| std::cmp::PartialOrd::partial_cmp(&x.0, &y.0).unwrap());
            for (i, event) in events.iter().enumerate() {
            // for (i, event) in  {
                println!("Event {}: {:?}", i, event);
            }
        }

        // Get minimum next event time
        let min_time = events.iter().min().unwrap().0;

        events.into_iter().filter(|x| x.0 == min_time).collect()
    }

    // roll state forward by time interval
    fn roll_forward_by(&mut self, time_delta: TimeDelta) {
        self.state.update(time_delta);
    }

    // Update the state instantaneously based on the type of event.
    fn instantaneous_update(&mut self, event: EventType) {
        self.handle_event(event)
    }

    fn get_state(&self) -> &Box<dyn State> {
        &self.state
    }

    fn handle_event(&mut self, event: EventType) {
        use EventType::*;
        match event {
            VehicleArrival => {
                self.new_vehicle();
                // EventResult::NewVehicle(self.new_vehicle())
            }
            VehicleAccelerate(idx) => {
                let vehicle = self.state.get_mut_vehicle(idx);
                vehicle.action(Action::Accelerate);
                // self.new_vehicle();
                // EventResult::NewVehicle(self.new_vehicle())
            }
            VehicleExit(idx) => {
                self.remove_vehicle(idx);
                // EventResult::RemoveVehicle
            }
            SpeedLimitReached(idx) => {
                let vehicle = self.state.get_mut_vehicle(idx);
                vehicle.set_speed(MAX_SPEED);
                vehicle.action(Action::StaticSpeed);
                // EventResult::VehicleChange(&*vehicle)
            }
            ZeroSpeedReached(idx) => {
                let vehicle = self.state.get_mut_vehicle(idx);
                vehicle.set_speed(0.0);
                vehicle.action(Action::StaticSpeed);
                // EventResult::VehicleChange(&*vehicle)
            }
            StaticSpeedReached(idx) => {
                let vehicle = self.state.get_mut_vehicle(idx);
                vehicle.action(Action::StaticSpeed);
            }
            EmergencyStop(idx) => {
                let vehicle = self.state.get_mut_vehicle(idx);
                vehicle.set_speed(0.0);
                vehicle.action(Action::StaticSpeed);
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
            PedestrianExit(id) => {
                self.remove_pedestrian(id);
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

    // fn get_state(&self) -> &Box<dyn State> {
    //     &self.state
    // }

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

        // Open a file for writing
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open("sim_states.json")
            .unwrap();

        let mut t: Time = 0;
        while t < self.end_time {
            // Debugging
            if self.verbose && t > 0  {
                raw_input();
            }

            // Get vec of events
            let next_events = self.next_events();

            // Get time of events
            let next_event_time = next_events[0].0;

            // Roll state forward to when events take place
            self.roll_forward_by(TimeDelta::new(next_event_time - t));

            // Loop over next events and apply state changes
            for (i, next_event) in next_events.into_iter().enumerate() {
                if self.verbose {
                    println!("Time: {}; Next event {}: {:?}", t, i, next_event);
                }
                self.instantaneous_update(next_event.1);
            }

            // Change t
            t = next_event_time;

            // Log state to file
            writeln!(file, "{}", &to_json_flat(self.get_state()).unwrap()).expect("Tried to write state.");

            // Temp prints
            println!("---");
            println!("State after update at time: {t}");
            let as_json= to_json(self.get_state()).unwrap();
            println!("{}", &as_json);
            println!("Total vehicles: {}", self.veh_counter+1);
            println!("Total pedestrians: {}", self.ped_counter+1);
            println!("Current vehicles: {}", self.state.get_vehicles().len());
            println!("Current pedestrians: {}", self.state.get_pedestrians().len());
        }
    }
}


#[cfg(test)]
mod tests {
    use core::time;
    use std::{collections::VecDeque, f32::EPSILON};
    use crate::vehicle::{DECCELERATION_VALUE, MAX_SPEED};
    use super::*;
    const MY_EPSILON: f32 = 0.001;

    fn dummy_sim(state: Box<dyn State >) -> EventDrivenSim  {
        let road = Road::new(100.0, Vec::new());
        // let state = Box::new(SimulatorState::new());
        EventDrivenSim::new(147, 0, 500_000, 0.1, 0.2, state,  road, false)
    }

    fn dummy_no_arrivals_sim(state: Box<dyn State >) -> EventDrivenSim  {
        let road = Road::new(100.0, Vec::new());
        // let state = Box::new(SimulatorState::new());
        EventDrivenSim::new(147, 0, 500_000, 0.0, 0.0, state, road, false)
    }

    #[test]
    fn test_set_state() {
        let state = Box::new(SimulatorState::new());
        let mut sim = dummy_sim(state);
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

        let state = Box::new(SimulatorState::new());
        let mut sim = dummy_sim(state);
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

        let state = Box::new(SimulatorState::new());
        let mut sim = dummy_sim(state);

        let ped_arrival_times = vec!(10000, 20000);
        let veh_arrival_times = vec!(12000, 21000);

        // Set the pedestrian & vehicle arrival times.
        sim.set_ped_arrival_times(ped_arrival_times);
        sim.set_veh_arrival_times(veh_arrival_times);

        let next_events = sim.next_events();
        let actual = next_events.first().unwrap();
        assert_eq!(actual.0, 10000);
    }

    #[test]
    fn test_vehicle_arrival_event() {

        let state = Box::new(SimulatorState::new());
        let mut sim = dummy_sim(state);

        let ped_arrival_times = vec!(5000, 7000);
        let veh_arrival_times = vec!(4000, 15000);

        // Set the pedestrian & vehicle arrival times.
        sim.set_ped_arrival_times(ped_arrival_times);
        sim.set_veh_arrival_times(veh_arrival_times);

        let next_events = sim.next_events();
        let actual = next_events.first().unwrap();
        assert_eq!(actual.0, 4000);
    }

    // TODO: update given change in event handling
    #[test]
    #[ignore]
    fn test_vehicle_stopping_event() {
        let speed = 10.0;
        let mut vehicles: VecDeque<Box<dyn Vehicle>> = VecDeque::new();
        vehicles.push_back(Box::new(Car::new(0u64, Direction::Up, speed, Action::Deccelerate)));


        let timestamp = 22 * TIME_RESOLUTION;
        let state = Box::new(SimulatorState::dummy(vehicles, VecDeque::new(), timestamp));

        let mut sim = dummy_sim(state);

        let next_events = sim.next_events();
        let actual = next_events.first().unwrap();
        assert_eq!(actual.0, timestamp + TimeDelta::floor((-1.0) * (speed / DECCELERATION_VALUE)));
    }

    #[test]
    fn test_case1_reaction_to_obstacle() {
        let mut v1 = Car::new(0, Direction::Up, 14., Action::StaticSpeed);
        let mut v2 = Car::new(1, Direction::Up, 0., Action::StaticSpeed);
        v1.set_position(0.);
        v2.set_position(100.);

        let mut vehicles: VecDeque<Box<dyn Vehicle>> = VecDeque::new();
        vehicles.push_back(Box::new(v1));
        vehicles.push_back(Box::new(v2));
        
        let state = Box::new(SimulatorState::dummy(vehicles, VecDeque::new(), 0));
        let sim = dummy_sim(state);
        let mv1 = sim.state.get_vehicle(0);
        let mv2 = sim.state.get_vehicle(1);
        let t_p = sim.time_to_obstacle_event::<dyn Obstacle>(mv1, mv2.as_obstacle(), false, false);
        assert!(f32::abs(t_p.unwrap() - 5.321429) < MY_EPSILON);

    }
    #[test]
    fn test_case1b_reaction_to_obstacle() {
        let mut v1 = Car::new(0, Direction::Up, 14., Action::StaticSpeed);
        let mut v2 = Car::new(1, Direction::Up, 10., Action::StaticSpeed);
        v1.set_position(80.);
        v2.set_position(100.);

        let mut vehicles: VecDeque<Box<dyn Vehicle>> = VecDeque::new();
        vehicles.push_back(Box::new(v1));
        vehicles.push_back(Box::new(v2));

        let state = Box::new(SimulatorState::dummy(vehicles, VecDeque::new(), 0));
        let sim = dummy_sim(state);
        let mv1 = sim.state.get_vehicle(0);
        let mv2 = sim.state.get_vehicle(1);
        let t_p = sim.time_to_obstacle_event::<dyn Obstacle>(mv1, mv2.as_obstacle(), false, false);
        assert!(f32::abs(t_p.unwrap() - 4.25) < MY_EPSILON);

    }
    #[test]
    fn test_case2_reaction_to_obstacle() {
        let mut v1 = Car::new(0, Direction::Up, 4., Action::Accelerate);
        let mut v2 = Car::new(1, Direction::Up, 0., Action::StaticSpeed);
        v1.set_position(0.);
        v2.set_position(20.);

        let mut vehicles: VecDeque<Box<dyn Vehicle>> = VecDeque::new();
        vehicles.push_back(Box::new(v1));
        vehicles.push_back(Box::new(v2));

        let state = Box::new(SimulatorState::dummy(vehicles, VecDeque::new(), 0));
        let sim = dummy_sim(state);
        let mv1 = sim.state.get_vehicle(0);
        let mv2 = sim.state.get_vehicle(1);
        let t_p = sim.time_to_obstacle_event::<dyn Obstacle>(mv1, mv2.as_obstacle(), false, false);
        
        assert!(f32::abs(t_p.unwrap() - 1.539638691) < MY_EPSILON);

    }
    #[test]
    fn test_case2b_reaction_to_obstacle() {
        let mut v1 = Car::new(0, Direction::Up, 14., Action::Accelerate);
        let mut v2 = Car::new(1, Direction::Up, 10., Action::StaticSpeed);
        v1.set_position(80.);
        v2.set_position(100.);

        let mut vehicles: VecDeque<Box<dyn Vehicle>> = VecDeque::new();
        vehicles.push_back(Box::new(v1));
        vehicles.push_back(Box::new(v2));

        let state = Box::new(SimulatorState::dummy(vehicles, VecDeque::new(), 0));
        let sim = dummy_sim(state);
        let mv1 = sim.state.get_vehicle(0);
        let mv2 = sim.state.get_vehicle(1);
        let t_p = sim.time_to_obstacle_event::<dyn Obstacle>(mv1, mv2.as_obstacle(), false, false);

        assert!(f32::abs(t_p.unwrap() - 1.539638691) < MY_EPSILON);

    }
    
    #[test]
    fn test_case3_reaction_to_obstacle() {
        let mut v1 = Car::new(0, Direction::Up, 14., Action::Accelerate);
        let mut v2 = Car::new(1, Direction::Up, 10., Action::Deccelerate);
        v1.set_position(0.);
        v2.set_position(25.);

        let mut vehicles: VecDeque<Box<dyn Vehicle>> = VecDeque::new();
        vehicles.push_back(Box::new(v1));
        vehicles.push_back(Box::new(v2));

        let state = Box::new(SimulatorState::dummy(vehicles, VecDeque::new(), 0));
        let sim = dummy_sim(state);
        let mv1 = sim.state.get_vehicle(0);
        let mv2 = sim.state.get_vehicle(1);
        let t_p = sim.time_to_obstacle_event::<dyn Obstacle>(mv1, mv2.as_obstacle(), false, false);

        assert!(f32::abs(t_p.unwrap() - 0.466481135) < MY_EPSILON);

    }
    #[test]
    fn test_case3_danger_reaction_to_obstacle() {
        let mut v1 = Car::new(0, Direction::Up, 14., Action::Accelerate);
        let mut v2 = Car::new(1, Direction::Up, 10., Action::Deccelerate);
        v1.set_position(0.);
        v2.set_position(10.);

        let mut vehicles: VecDeque<Box<dyn Vehicle>> = VecDeque::new();
        vehicles.push_back(Box::new(v1));
        vehicles.push_back(Box::new(v2));

        let state = Box::new(SimulatorState::dummy(vehicles, VecDeque::new(), 0));
        let sim = dummy_sim(state);
        let mv1 = sim.state.get_vehicle(0);
        let mv2 = sim.state.get_vehicle(1);
        let t_p = sim.time_to_obstacle_event::<dyn Obstacle>(mv1, mv2.as_obstacle(), false, false);

        // Negative time as already will end up in danger zone
        assert!(f32::abs(t_p.unwrap() - -0.124099041) < MY_EPSILON);

    }
    #[test]
    fn test_vehicle_speed_limit_event() {

        let speed = 10.0;
        let vehicles: Vec<Box<dyn Vehicle>> = vec!(Box::new(Car::new(0u64, Direction::Up, speed, Action::Accelerate)));

        let timestamp = 11 * TIME_RESOLUTION;
        let state = Box::new(SimulatorState::dummy(vehicles.into(), VecDeque::new(), timestamp));

        let mut sim = dummy_sim(state);
        // sim.set_state(Box::new(state));

        // let veh = sim.state.get_vehicle(0);
        // let pos = veh.get_position(sim.get_road(), &Direction::Up);
        // let speed = veh.get_speed();
        // let acc = veh.get_acceleration();
        // let time = sim.time_to_obstacle_event::<dyn Obstacle>(veh, sim.road.get_exit(), false).unwrap();
        // println!("time delta: {:?}, exit: {:?}, pos:{:?}, speed: {:?}, acc: {:?}",
        // time, sim.road.get_exit(), pos, speed, acc);
        // println!("time delta: {:?}, exit: {:?}, rel_pos:{:?}, rel_speed: {:?}, rel_acc: {:?}",
        // time, sim.road.get_exit(),
        // veh.relative_position(sim.road.get_exit(), sim.get_road()),
        // veh.relative_speed(sim.road.get_exit()),
        // veh.relative_acceleration(sim.road.get_exit())
        // );

        let next_events = sim.next_events();
        let actual = next_events.first().unwrap();
        // println!("{:?}, {}", actual, timestamp, );
        assert_eq!(actual.0, timestamp + TimeDelta::floor((MAX_SPEED - speed) / ACCELERATION_VALUE));
    }
    
    #[test]
    #[ignore]
    fn test_integration_two_zebras() {

        let crossings = vec![
	        (Crossing::Zebra { id: 0, cross_time: TimeDelta::from_secs(10) }, 170.0),
	        (Crossing::Zebra { id: 1, cross_time: TimeDelta::from_secs(10) }, 290.0),
	    ];

        let road = Road::new(300.0f32, crossings);
        let state = Box::new(SimulatorState::new());
        let mut sim = EventDrivenSim::new(12345, 0, 500_000, 0.1, 0.2, state, road, false);

        sim.run();
    
    
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

    // }
    //
    // Test the static helper functions.
    //

    

// ------
// let as_json= to_json(self.get_state()).unwrap();
// println!("{}", &as_json);
// -----

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

//         let actual = sim.next_events();
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

//         let actual = sim.next_events();
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

//         let actual= sim.next_events();
//         assert_eq!(actual.0, timestamp + TimeDelta::floor((-1.0) * (speed / DECCELERATION_VALUE)));
//     }

//     #[test]
//     fn test_vehicle_speed_limit_event() {

//         let speed = 10.0;
//         let vehicles: Vec<Box<dyn Vehicle>> = vec!(Box::new(Car::new(0u64, Direction::Up, speed, Action::Accelerate)));

//         let timestamp = 11 * TIME_RESOLUTION;
//         let state = SimulatorState::dummy(vehicles.into(), VecDeque::new(), timestamp);

//         let mut sim = dummy_sim();
//         sim.set_state(Box::new(state));

//         let actual = sim.next_events();
//         assert_eq!(actual.0, timestamp + TimeDelta::floor((MAX_SPEED - speed) / ACCELERATION_VALUE));
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
