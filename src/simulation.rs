
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


pub trait Simulation  {

    // get time interval until next event
    fn next_event(&mut self) -> Event;

    // roll simulation forward by time interval
    fn roll_forward_by(&mut self, time_delta: TimeDelta);

    // update simulation state
    fn instantaneous_update(&mut self, event_type: EventType);

    fn get_state(&self) -> &Box<dyn State> ;

    fn handle_event(&mut self, event: EventType);

    fn get_road(&self) -> &Road;

    fn run(&mut self) -> ();
}

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
