use std::time::{Duration, Instant};
use crate::road::Crossing;

// Notes:
// How long does a person take to cross the road?
// 1. Person spawns at crossing at `arrival_time` e.g. t = t_a
// 2. Wait for a time t_w
// 3. Crossing at time t_a + t_w <=  t = t_a + t_w + t_c

// Actions:
// A. Car must consider person on road between time:
//    t_a + t_w  <= t < t_a + t_c
// B. Car must consider  slowing down for person waiting at zebra:
//    t_a <= t < t_w

// TODO: Sequence of pedestrians with arrival times need to be incorporated
// into state struct. E.g. `generate_pedestrian()` to be implemented.

pub trait Person {
    fn location(&self) -> &Crossing;
    fn arrival_time(&self) -> Instant;
}

pub struct Pedestrian<'a> {
    location: &'a Crossing,
    arrival_time: Instant,
}

impl Person for Pedestrian<'_> {
    fn location(&self) -> &Crossing {
        self.location
    }

    fn arrival_time(&self) -> Instant {
        self.arrival_time
    }
}

impl Pedestrian<'_> {
    fn new(location: &Crossing, arrival_time: Instant) -> Pedestrian {
        Pedestrian {
            location,
            arrival_time,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pedestrian_location() {
        let test_pelican = Crossing::pelican(25.0);
        let test_pedestrian = Pedestrian::new(&test_pelican, Instant::now());
        assert_eq!(test_pedestrian.location(), &test_pelican);
    }

    #[test]
    fn test_pedestrian_arrival() {
        let test_zebra = Crossing::zebra(25.0);
        let test_pedestrian = Pedestrian::new(&test_zebra, Instant::now());
        let complete_time = test_pedestrian.arrival_time + test_zebra.stop_time();
        assert_eq!(
            test_pedestrian.arrival_time() + test_zebra.stop_time(),
            complete_time
        );
    }
}
