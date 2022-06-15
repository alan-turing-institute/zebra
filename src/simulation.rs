use std::cmp;
use rand_distr::{Exp, Distribution};
use rand::{SeedableRng}; // SeedableRng needed for the seed_from_u64 method.
use rand::rngs::StdRng;

use crate::Time;
use crate::road::{GO_TIME, WAIT_TIME, CROSSING_TIME, Road};
use crate::time::TIME_RESOLUTION;

pub struct Simulation {

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



// Function to convert a set of pelican arrival times to actual (zebra) crossing
// times
fn pelican_to_zebra(pel_times: &Vec<Time>) -> Vec<Time> {
    // Get Time versions of deltas
    let go_time: Time = GO_TIME.into();
    let cross_time: Time = CROSSING_TIME.into();
    let wait_time: Time = WAIT_TIME.into();

    // println!("go:{}, cross:{}, wait:{}", go_time, cross_time, wait_time);
    
    // Make vector for zebra times
    let mut zeb_times = Vec::<Time>::new();

    // Initialise next and previous pelican crossing times
    let mut next_cross_time =  pel_times.first().unwrap() + wait_time;
    
    // Loop over pelican times
    for &pel_time in pel_times {
	// Store next_cross_time into previous_cross_time
	let prev_cross_time: Time = next_cross_time;

	// Get the difference of the next time to the previous time
	let diff_from_previous = pel_time - prev_cross_time;

	// If arrival time is after the next crossing time, update
	if pel_time > next_cross_time {
	    next_cross_time = match diff_from_previous {
		x if x < cross_time => prev_cross_time + cross_time + go_time,
		_ => cmp::max(pel_time + wait_time, prev_cross_time + cross_time + go_time)
	    };
	}

	// Push new time to back
	zeb_times.push(next_cross_time);
    }
    zeb_times
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
        let end_time = 10 * TIME_RESOLUTION;
        let actual = arrival_times(&0, &end_time, 0.2, &mut rng);
        assert_eq!(actual.len(), 2);

        // Check that all arrivals occur before the simulation end time.
        assert!(actual.iter().max().unwrap() <= &end_time);
    }

    #[test]
    fn test_pelican_to_zebra() {
	// Make pelican times
	let actual_pel: Vec<Time> = vec![1, 2,  9, 12, 14, 17, 21, 22, 60].iter().map(|x| Time::from(x * TIME_RESOLUTION)).collect();

	// Make actual zebra times
	let actual_zeb: Vec<Time> = vec![6, 6, 21, 21, 21, 21, 21, 36, 65].iter().map(|x| Time::from(x * TIME_RESOLUTION)).collect();

	// Convert to zebra times
	let zeb: Vec<Time> = pelican_to_zebra(&actual_pel);


	// Print for viewing with: cargo test -- --nocapture
	for (a, b) in actual_pel.iter().zip(zeb.clone()) {
	    println!("{}, {}", *a, b);
	}
	
	assert_eq!(actual_zeb.len(), zeb.len());
	assert_eq!(actual_zeb.iter().zip(zeb).all(|(a, b)| *a == b), true);
    }

    // TODO: write additional tests for pelican_to_zebra()
}
