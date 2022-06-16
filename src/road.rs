
use serde::{Serialize, Deserialize};

use crate::obstacle::Obstacle;
use crate::{ID, TimeDelta};
use crate::config::get_zebra_config;
use crate::{Length, Position};
// type Length = f32;
// type Position = Length;

#[derive(Copy,Clone,Serialize,Deserialize)]
pub enum Direction {
    Up,
    Down
}

#[derive(Debug, Copy, PartialEq, Clone, Serialize, Deserialize)]
pub enum Crossing {
    Zebra {
	    id: ID, // unique identifier
        cross_time: TimeDelta
    },
    Pelican {
	    id: ID, // unique identifier
        stop_time: TimeDelta, // time traffic is stopped
        wait_time: TimeDelta, // time from pressing button to stop
        go_time: TimeDelta // min time traffic flow before a stop can occur
    }
}

pub const CROSSING_TIME: TimeDelta = TimeDelta::from_secs(10);
pub const WAIT_TIME: TimeDelta = TimeDelta::from_secs(5);
pub const GO_TIME: TimeDelta = TimeDelta::from_secs(5);

// TODO: update once config is implemented and can be loaded from
pub fn generate_crossings() -> Vec<(Crossing, Position)> {
    let mut crossings: Vec<(Crossing, Position)> = Vec::new();
    crossings.push((Crossing::pelican(0), 100.0));
    crossings.push((Crossing::zebra(1), 500.0));
    crossings.push((Crossing::pelican(2), 800.0));
    crossings
}

impl Crossing {

    pub fn zebra(id: ID) -> Crossing
    {
        Crossing::Zebra { id, cross_time: CROSSING_TIME}
    }

    pub fn pelican(id: ID) -> Crossing
    {
        Crossing::Pelican {
	        id,
            stop_time: CROSSING_TIME,
            wait_time: WAIT_TIME,
            go_time: GO_TIME
        }
    }

    pub fn set_id(&mut self, new_id: ID) {
        match self {
            Crossing::Zebra {ref mut id, ..} => *id = new_id,
            Crossing::Pelican {ref mut id, ..} => *id = new_id,
        };
    }

    pub fn get_id(&self) -> ID {
	match self {
            Crossing::Zebra {id, ..} => *id,
            Crossing::Pelican { id, ..} => *id
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

impl Obstacle for Crossing {

    fn get_position(&self, road: &Road, direction: &Direction) -> f32 {


        // Get the ID of this crossing.
        // let id = &self.get_id();

        // TODO.
        // road.get_crossing_position(id, direction)
        0.0
    }

    fn get_speed(&self) -> f32 {
        0.0
    }

    fn get_acceleration(&self) -> f32 {
        0.0
    }
}

pub struct Road {
    length: Length,
    crossings_up: Vec<(Crossing, Position)>,
    crossings_down: Vec<(Crossing, Position)>
}

impl Road {

    // Here the position of the crossings is assumed to be in the `Up` direction.
    pub fn new() -> Road {

    // Load from zebra.toml
    let config = get_zebra_config();

    // Assign length from config
    let length = config.road_length;

    // Load in crossings
    let mut crossings: Vec<(Crossing, Position)> = Vec::new();
    for &crossing in &config.zebra_crossings {
        crossings.push(
            (Crossing::zebra(u64::max_value()), crossing)
        );
    }
    for &crossing in &config.pelican_crossings {
        crossings.push(
            (Crossing::pelican(u64::max_value()), crossing)
        );
    }

    // Sort crossings by position (second element of tuple)
    crossings.sort_by(|x, y| std::cmp::PartialOrd::partial_cmp(&x.1, &y.1).unwrap());

    let mut i = 0;
    for (crossing, position) in &mut crossings {
        crossing.set_id(i as ID);
        i += 1;
    }

    // Check valid crossings
    for (_, position) in &crossings {
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
    use std::any::Any;

    use super::*;

    #[test]
    fn test_generate_crossings() {
        let test_crossings = generate_crossings();
        assert_eq!(&test_crossings, &[
            (Crossing::pelican(0), 100.0),
            (Crossing::zebra(1), 500.0),
            (Crossing::pelican(2), 800.0),
        ]);
    }

    #[test]
    fn test_stop_time_zebra() {
        let test_zebra = Crossing::zebra(0);
        assert_eq!(test_zebra.stop_time(), CROSSING_TIME);
    }

    #[test]
    fn test_stop_time_pelican() {
        let test_pelican = Crossing::pelican(0);
        assert_eq!(test_pelican.stop_time(), CROSSING_TIME);
    }

    #[test]
    fn test_arrive_to_stop_zebra() {
        let test_zebra = Crossing::zebra(0);
        assert_eq!(test_zebra.arrival_to_stop_time(), TimeDelta::from(0));
    }

    #[test]
    fn test_arrive_to_stop_pelican() {
        let test_pelican = Crossing::pelican(0);
        assert_eq!(test_pelican.arrival_to_stop_time(), WAIT_TIME);
    }

    #[test]
    fn test_road_constructor() {
        let road = Road::new();    
    }

    // TODO: removed test from here as should be tested in config for valid entries.
    // #[test]
    // #[should_panic]
    // fn test_road_constructor_panics() {
    //     // Should panic due to invalid crossing position.
    //     // let crossings = vec![(Crossing::Zebra {id: 0, cross_time: TimeDelta::from_secs(25) }, 30.0)];
    //     let road = Road::new();
    //     let length = road.get_length();
    //     for (crossing, position) in road.get_crossings(Direction::Up).into_iter() {
    //         if *position < 0.0 || *position > length {
    //             panic!();
    //         }
    //     }
    // }

    #[test]
    fn test_road_get_length() {
        let test_road = Road::new();
        let test_config = get_zebra_config();
        assert_eq!(test_road.get_length(), test_config.road_length);
    }

    #[test]
    fn test_crossing_get_id() {
        let test_pelican = Crossing::pelican(0);
	    assert_eq!(test_pelican.get_id(), 0);
	    let test_zebra = Crossing::zebra(1);
        assert_eq!(test_zebra.get_id(), 1);
    }

    #[test]
    fn test_road_get_crossings() {
        
        let road = Road::new();

        // IDs count monotonically up when direction is Up, and check position is increasing
        let crossings = road.get_crossings(Direction::Up);
        let mut previous_position = 0.;
        for (i, &(crossing, position)) in crossings.into_iter().enumerate() {
            assert_eq!(crossing.get_id(), i as ID);
            assert!(position > previous_position);
            previous_position = position;
        }

        // IDs count monotonically down when direction is Down
        // Check position is increasing
        let crossings = road.get_crossings(Direction::Down);
        let mut previous_position = 0.;
        let mut i: usize = crossings.len();
        for &(crossing, position) in crossings.into_iter() {
            i -= 1;
            assert_eq!(crossing.get_id(), i as ID);
            assert!(position > previous_position);
            previous_position = position;
        }

	    
        // assert_eq!(
        //     road.get_crossings(direction),
        //     &[
        //     (Crossing::Zebra { id: 0, cross_time: TimeDelta::from_secs(25)}, 10.0),
        //     (Crossing::Zebra { id: 1, cross_time: TimeDelta::from_secs(10)}, 13.0),
        // ]);
	    // let direction = Direction::Down;
        // assert_eq!(
	    // road.get_crossings(direction),
	    // &[
		// (Crossing::Zebra { id: 1, cross_time: TimeDelta::from_secs(10)}, 17.0),
		// (Crossing::Zebra { id: 0, cross_time: TimeDelta::from_secs(25)}, 20.0),
	    // ]);
    }
}
