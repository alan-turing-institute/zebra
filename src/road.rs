
use serde::{Serialize, Deserialize};

use crate::TimeDelta;

type Length = f32;
type Position = Length;

pub enum Direction {
    Up,
    Down
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Crossing {
    Zebra {
        cross_time: TimeDelta
    },
    Pelican {
        stop_time: TimeDelta, // time traffic is stopped
        wait_time: TimeDelta, // time from pressing button to stop
        go_time: TimeDelta // min time traffic flow before a stop can occur
    }
}

pub const CROSSING_TIME: TimeDelta = TimeDelta::from_secs(10);
pub const WAIT_TIME: TimeDelta = TimeDelta::from_secs(5);
pub const GO_TIME: TimeDelta = TimeDelta::from_secs(5);

impl Crossing {

    pub fn zebra() -> Crossing
    {
        Crossing::Zebra { cross_time: CROSSING_TIME}
    }

    pub fn pelican() -> Crossing
    {
        Crossing::Pelican {
            stop_time: CROSSING_TIME,
            wait_time: WAIT_TIME,
            go_time: GO_TIME
        }
    }

    pub fn stop_time(&self) -> TimeDelta {
        match self {
            Crossing::Zebra {cross_time, ..} => *cross_time,
            Crossing::Pelican { stop_time, ..} => *stop_time
        }
    }

    pub fn arrival_to_stop_time(&self) -> TimeDelta {
        match self {
            Crossing::Zebra {..} => TimeDelta::from(0),
            Crossing::Pelican {wait_time, ..} => *wait_time
        }
    }

    pub fn min_time_to_next_stop(&self) -> TimeDelta {
        match self {
            Crossing::Zebra {..} => TimeDelta::from(0),
            Crossing::Pelican {go_time,..} => *go_time
        }
    }

}

pub struct Road {
    length: Length,
    crossings_up: Vec<(Crossing, Position)>,
    crossings_down: Vec<(Crossing, Position)>
}

impl Road {

    pub fn new(length: Length, crossings: Vec<(Crossing, Position)>) -> Road {

        for (_, position) in crossings.iter() {
            assert!(0.0 <= *position && *position <= length);
        }

	let crossings_up: Vec<(Crossing, Position)> = crossings.clone();
	let mut crossings_down: Vec<(Crossing, Position)> = Vec::new();
	for (crossing, position) in crossings.into_iter().rev() {
	    crossings_down.push((crossing, length - position));
	}
        Road { length, crossings_up, crossings_down }
    }

    pub fn get_length(&self) -> Length {
        self.length
    }

    pub fn get_crossings(&self, direction: Direction) -> &[(Crossing, Position)]
    {
	match direction {
	    Direction::Up => &self.crossings_up,
	    Direction::Down => &self.crossings_down 
	}	
    }

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stop_time_zebra() {
        let test_zebra = Crossing::zebra();
        assert_eq!(test_zebra.stop_time(), CROSSING_TIME);
    }

    #[test]
    fn test_stop_time_pelican() {
        let test_pelican = Crossing::pelican();
        assert_eq!(test_pelican.stop_time(), CROSSING_TIME);
    }

    #[test]
    fn test_arrive_to_stop_zebra() {
        let test_zebra = Crossing::zebra();
        assert_eq!(test_zebra.arrival_to_stop_time(), TimeDelta::from(0));
    }

    #[test]
    fn test_arrive_to_stop_pelican() {
        let test_pelican = Crossing::pelican();
        assert_eq!(test_pelican.arrival_to_stop_time(), WAIT_TIME);
    }

    #[test]
    fn test_road_constructor() {

        Road::new(20.0f32, Vec::new());

        let crossings = vec![(Crossing::Zebra { cross_time: TimeDelta::from_secs(25) }, 10.0)];
        Road::new(20.0f32, crossings);
    }

    #[test]
    #[should_panic]
    fn test_road_constructor_panics() {

        // Should panic due to invalid crossing position.
        let crossings = vec![(Crossing::Zebra { cross_time: TimeDelta::from_secs(25) }, 30.0)];
        Road::new(20.0f32, crossings);
    }

    #[test]
    fn test_road_get_length() {
        let test_road = Road::new(20.0f32, Vec::new());
        assert_eq!(test_road.get_length(), 20.0f32);
    }

    #[test]
    fn test_road_get_crossings() {

        let crossings = vec![
	    (Crossing::Zebra { cross_time: TimeDelta::from_secs(25) }, 10.0),
	    (Crossing::Zebra { cross_time: TimeDelta::from_secs(10) }, 13.0),
	];
        let road = Road::new(30.0f32, crossings);

	let direction = Direction::Up;
        assert_eq!(
	    road.get_crossings(direction),
	    &[
		(Crossing::Zebra { cross_time: TimeDelta::from_secs(25)}, 10.0),
		(Crossing::Zebra { cross_time: TimeDelta::from_secs(10)}, 13.0),
	    ]);

	let direction = Direction::Down;
        assert_eq!(
	    road.get_crossings(direction),
	    &[
		(Crossing::Zebra { cross_time: TimeDelta::from_secs(10)}, 17.0),
		(Crossing::Zebra { cross_time: TimeDelta::from_secs(25)}, 20.0),
	    ]);
    }
}
