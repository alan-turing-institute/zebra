
use rand_distr::{Exp, Distribution};
use rand::{SeedableRng}; // SeedableRng needed for the seed_from_u64 method.
use rand::rngs::StdRng;

use crate::state::Time;

struct Simulation {

    seed: u64,

    start_time: Time,
    end_time: Time,

    ped_arrival_rate: f32,
    veh_arrival_rate: f32,

    pub ped_arrival_times: Vec<f32>,
    pub veh_arrival_times: Vec<f32>
}

impl Simulation {

    pub fn new(seed: u64,
        start_time: Time,
        end_time: Time,
        ped_arrival_rate: f32,
        veh_arrival_rate: f32) -> Simulation {

        assert!(end_time > start_time);

        // Set the random seed.
        // See https://stackoverflow.com/questions/59020767/how-can-i-input-an-integer-seed-for-producing-random-numbers-using-the-rand-crat
        let mut rng = StdRng::seed_from_u64(seed);

        // Generate pedestrian & vehicle arrival times.
        let ped_arrival_times = arrival_times(&start_time, &end_time, ped_arrival_rate, &mut rng);
        let veh_arrival_times = arrival_times(&start_time, &end_time, veh_arrival_rate, &mut rng);

        let sim = Simulation {
            seed: seed,
            start_time: start_time,
            end_time: end_time,
            ped_arrival_rate: ped_arrival_rate,
            veh_arrival_rate: veh_arrival_rate,
            ped_arrival_times: ped_arrival_times,
            veh_arrival_times: veh_arrival_times
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
    while &t < end_time {
        t = t + interarrival_time(arrival_rate, rng);
        ret.push(t);
    }
    ret
}

fn interarrival_time(arrival_rate: f32, rng: &mut StdRng) -> Time {
    let exp = Exp::new(arrival_rate).unwrap(); // see https://docs.rs/rand_distr/0.2.1/rand_distr/struct.Exp.html
    exp.sample(rng)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interarrival_time() {

        // Set the random seed.
        let mut rng = StdRng::seed_from_u64(147);
        let actual = interarrival_time(2.0, &mut rng);
        let expected = 0.26590475;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_arrival_times() {

        // Set the random seed.
        let mut rng = StdRng::seed_from_u64(147);

        // With this seed, there are 3 arrivals in 10 seconds.
        assert_eq!(arrival_times(&0.0, &10.0, 0.2, &mut rng).len(), 3);
    }
}