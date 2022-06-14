
use rand_distr::{Exp, Distribution};
use rand::{SeedableRng}; // SeedableRng needed for the seed_from_u64 method.
use rand::rngs::StdRng;

use crate::Time;
use crate::road::Road;
use crate::time::TIME_RESOLUTION;

struct Simulation {

    seed: u64,

    start_time: Time,
    end_time: Time,

    ped_arrival_rate: f32,
    veh_arrival_rate: f32,

    pub ped_arrival_times: Vec<Time>,
    pub veh_arrival_times: Vec<Time>,

    road: Road
}

impl Simulation {

    pub fn new(seed: u64,
        start_time: Time,
        end_time: Time,
        ped_arrival_rate: f32,
        veh_arrival_rate: f32,
        road: Road) -> Simulation {

        assert!(end_time > start_time);

        // Set the random seed.
        // See https://stackoverflow.com/questions/59020767/how-can-i-input-an-integer-seed-for-producing-random-numbers-using-the-rand-crat
        let mut rng = StdRng::seed_from_u64(seed);

        // Generate pedestrian & vehicle arrival times.
        let ped_arrival_times = arrival_times(&start_time, &end_time, ped_arrival_rate, &mut rng);
        let veh_arrival_times = arrival_times(&start_time, &end_time, veh_arrival_rate, &mut rng);

        let sim = Simulation {
            seed,
            start_time,
            end_time,
            ped_arrival_rate,
            veh_arrival_rate,
            ped_arrival_times,
            veh_arrival_times,
            road
        };

        // TODO. Construct initial (empty) state at time 0.
        // ...

        sim
    }

    // pub fn current_state() -> State {

    // }
}

fn arrival_times(start_time: &Time, end_time: &Time, arrival_rate: f32, rng: &mut StdRng) -> Vec<Time> {

    let mut ret = Vec::new();
    let mut t = start_time.clone();
    loop {
        t = t + interarrival_time(arrival_rate, rng);
        if &t > end_time { break; }
        ret.push(t);
    }
    ret
}

fn interarrival_time(arrival_rate: f32, rng: &mut StdRng) -> Time {
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
        let actual = arrival_times(&0, &10000, 0.2, &mut rng);
        assert_eq!(actual.len(), 2);

        // TODO NEXT: FIX ERROR: "the trait bound `f32: Ord` is not satisfied"
        // - Additional argument for using integer time?
        // assert!(actual.iter().max().unwrap() <= &10.0);
    }
}