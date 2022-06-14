
// use std::time::{Duration};
use serde::{Serialize, Deserialize};

use crate::TimeDelta;

type Length = f32;
type Position = Length;

pub enum Direction {
    Up,
    Down
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Crossing {
    Zebra {
        // position: Length,  // position along road
        cross_time: TimeDelta
    },
    Pelican {
        // position: Length, // position along road
        stop_time: TimeDelta, // time traffic is stopped
        wait_time: TimeDelta, // time from pressing button to stop
        go_time: TimeDelta // min time traffic flow before a stop can occur
    }
}

pub const CROSSING_TIME: TimeDelta = TimeDelta::from_secs(10);
pub const WAIT_TIME: TimeDelta = TimeDelta::from_secs(5);
pub const GO_TIME: TimeDelta = TimeDelta::from_secs(5);

impl Crossing {

    // pub fn zebra(position: Length) -> Crossing
    pub fn zebra() -> Crossing
    {
        Crossing::Zebra { cross_time: CROSSING_TIME}
    }

    // pub fn pelican(position: Length) -> Crossing
    pub fn pelican() -> Crossing
    {
        Crossing::Pelican {
            // position,
            stop_time: CROSSING_TIME,
            wait_time: WAIT_TIME,
            go_time: GO_TIME
        }
    }

    // pub fn get_position(&self) -> Length {
    //     match self {
    //         Crossing::Zebra { position , ..} => *position,
    //         Crossing::Pelican { position, ..} => *position
    //     }
    // }

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
    crossings: Vec<(Crossing, Position)>
}

impl Road {

    pub fn new(length: Length, crossings: Vec<(Crossing, Position)>) -> Road {

        for (_, position) in crossings.iter() {
            assert!(0.0 <= *position && *position <= length);
        }
        Road { length, crossings }
    }

    pub fn get_length(&self) -> Length {
        self.length
    }

    pub fn get_crossings(&self) -> &[(Crossing, Position)]
    {
       &self.crossings
    }

}



#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_get_position_zebra() {
    //     let test_zebra = Crossing::zebra(25.0);
    //     assert_eq!(test_zebra.get_position(), 25.0f32);
    // }

    // #[test]
    // fn test_get_position_pelican() {
    //     let test_pelican = Crossing::pelican(25.0);
    //     assert_eq!(test_pelican.get_position(), 25.0f32);
    // }

    #[test]
    fn test_stop_time_zebra() {
        // let test_zebra = Crossing::zebra(25.0);
        let test_zebra = Crossing::zebra();
        assert_eq!(test_zebra.stop_time(), CROSSING_TIME);
    }

    #[test]
    fn test_stop_time_pelican() {
        // let test_pelican = Crossing::pelican(25.0);
        let test_pelican = Crossing::pelican();
        assert_eq!(test_pelican.stop_time(), CROSSING_TIME);
    }

    #[test]
    fn test_arrive_to_stop_zebra() {
        // let test_zebra = Crossing::zebra(25.0);
        let test_zebra = Crossing::zebra();
        assert_eq!(test_zebra.arrival_to_stop_time(), TimeDelta::from(0));
    }

    #[test]
    fn test_arrive_to_stop_pelican() {
        // let test_pelican = Crossing::pelican(25.0);
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

        // Invalid crossing position.
        let crossings = vec![(Crossing::Zebra { cross_time: TimeDelta::from_secs(25) }, 30.0)];
        Road::new(20.0f32, crossings);
    }

    #[test]
    fn test_road_get_length() {
        let test_road = Road { length: 20.0f32, crossings: Vec::new() };
        assert_eq!(test_road.get_length(), 20.0f32);
    }

    #[test]
    fn test_road_get_crossings() {

        // let test_road = Road { length: 20.0f32,
        //     crossings: vec![Crossing::Zebra { position: 10.0f32, cross_time: TimeDelta::from_secs(25) }] };

        // assert_eq!(test_road.get_crossings(), &[Crossing::Zebra { position: 10.0f32, cross_time: TimeDelta::from_secs(25)}]);

        let crossings = vec![(Crossing::Zebra { cross_time: TimeDelta::from_secs(25) }, 10.0)];
        let road = Road { length: 20.0f32, crossings };

        assert_eq!(road.get_crossings(), &[(Crossing::Zebra { cross_time: TimeDelta::from_secs(25)}, 10.0)]);
    }
}
