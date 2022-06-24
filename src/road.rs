
use serde::{Serialize, Deserialize};

use crate::obstacle::Obstacle;
use crate::{ID, TimeDelta};
use crate::config::{get_zebra_config};
use crate::{Length, Position};
use std::rc::Rc;

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

        // Use the ID of this crossing to get its position in the road.
        let id = &self.get_id();
        road.get_crossing_position(id, *direction)
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
    crossings_up: Vec<(Rc<Crossing>, Position)>,
    crossings_down: Vec<(Rc<Crossing>, Position)>
}

fn get_crossings_up_and_down(length: Length, crossings: Vec<(Crossing, Position)>) -> (Vec<(Rc<Crossing>, Position)>, Vec<(Rc<Crossing>, Position)>) {
    // Make vec of crossings for up
    let mut crossings_up: Vec<(Rc<Crossing>, Position)> = Vec::new();
    for i in 0..crossings.len() {
        let (crossing, pos) = crossings[i];
        crossings_up.push((Rc::new(crossing), pos));
    }

    // Make vec of crossings for down with reference to crossings in up
    let mut crossings_down: Vec<(Rc<Crossing>, Position)> = Vec::new();
    for i in 0..crossings.len() {
        let i_rev = crossings.len() - 1 - i;
        let (crossing, pos) = &crossings_up[i_rev];
        crossings_down.push((Rc::clone(crossing), length-pos));
    }
    (crossings_up, crossings_down)
}

impl Road {
    // Here the position of the crossings is assumed to be in the `Up` direction.
    pub fn new(length: Length, crossings: Vec<(Crossing, Position)>) -> Road {
        for (_, position) in crossings.iter() {
            assert!(0.0 <= *position && *position <= length);
        }

        // Assert the ordering of the ids is correct.
        let mut i: ID = 0 as ID;
        for (crossing, _) in &crossings {
            assert_eq!(crossing.get_id(), i);
            i += 1;
        }

        // Make vec of crossings for up and down with Rc
        let (crossings_up, crossings_down) = get_crossings_up_and_down(length, crossings);        

        Road { length, crossings_up, crossings_down}
    }

    // Here the position of the crossings is assumed to be in the `Up` direction.
    pub fn config_new() -> Road {

        // Load from zebra.toml
        let config = get_zebra_config();

        // Assign length from config
        let length = config.road_length;

        println!("{:?}", length);

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
        for (crossing, _) in &mut crossings {
            crossing.set_id(i as ID);
            i += 1;
        }

        // Check valid crossings
        for (_, position) in &crossings {
            assert!(0.0 <= *position && *position <= length);
        }

        // Make vec of crossings for up and down with Rc
        let (crossings_up, crossings_down) = get_crossings_up_and_down(length, crossings);

        Road { length, crossings_up, crossings_down }
    }

    pub fn get_length(&self) -> Length {
        self.length
    }

    pub fn get_crossings(&self, direction: &Direction) -> &[(Rc<Crossing>, Position)]
    {
        match direction {
            Direction::Up => &self.crossings_up,
            Direction::Down => &self.crossings_down
        }
    }

    pub fn get_crossing_position(&self, id: &ID, direction: Direction) -> f32 {

        // TODO. If the vector of crossings is in order of ID,
        // there's a much quicker way of doing this.
        for (crossing, position) in self.get_crossings(&direction).iter() {
            if &crossing.get_id() == id {
                return *position
            }
        }
        panic!("Crossing ID not found: {}", id);
    }

}


#[cfg(test)]
mod tests {
    use super::*;

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
        let road = Road::config_new();    
    }

    #[test]
    #[should_panic]
    fn test_road_constructor_panics() {
        // Should panic due to invalid crossing position.
        let crossings = vec![(Crossing::Zebra {id: 0, cross_time: TimeDelta::from_secs(25) }, 30.0)];
        Road::new(20.0f32, crossings);
    }

    #[test]
    fn test_road_get_length_config() {
        let test_road = Road::config_new();
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
    fn test_get_position() {

        let crossings = vec![
            (Crossing::Zebra { id: 0, cross_time: TimeDelta::from_secs(25) }, 10.0),
            (Crossing::Zebra { id: 1, cross_time: TimeDelta::from_secs(10) }, 13.0),
        ];
        let road = Road::new(30.0f32, crossings);

        let (crossing, _) = &road.get_crossings(&Direction::Up)[0];
        assert_eq!(crossing.get_position(&road, &Direction::Up), 10.0);
        assert_eq!(crossing.get_position(&road, &Direction::Down), 30.0 - 10.0);

        let (crossing, _) = &road.get_crossings(&Direction::Up)[1];
        assert_eq!(crossing.get_position(&road, &Direction::Up), 13.0);
        assert_eq!(crossing.get_position(&road, &Direction::Down), 30.0 - 13.0);
    }


    #[test]
    fn test_road_get_crossings() {
        let road = Road::config_new();

        // IDs count monotonically up when direction is Up, and check position is increasing
        let crossings = road.get_crossings(&Direction::Up);
        let mut previous_position = 0.;
        for i in 0..crossings.len() {
            let (crossing, position) = &crossings[i];
            assert_eq!(crossing.get_id(), i as ID);
            assert!(*position > previous_position);
            previous_position = *position;
        }

        // IDs count monotonically down when direction is Down
        // Check position is increasing
        let crossings = road.get_crossings(&Direction::Down);
        previous_position = 0.;
        for i in 0..crossings.len() {
            let i_rev = crossings.len() - 1 - i;
            let (crossing, position) = &crossings[i];
            assert_eq!(crossing.get_id(), i_rev as ID);
            assert!(*position > previous_position);
            previous_position = *position;
        }
    }

    #[test]
    fn test_road_get_crossings_rc_equivalence() {
        // Make test_road
        let crossings = vec![
	        (Crossing::Zebra { id: 0, cross_time: TimeDelta::from_secs(25) }, 10.0),
	        (Crossing::Zebra { id: 1, cross_time: TimeDelta::from_secs(10) }, 13.0),
	    ];
        let test_road = Road::new(30.0f32, crossings);

        // Check crossings_up and crossings_down have same shared pointer (Rc)
        // when reversed
        let crossings_up = test_road.get_crossings(&Direction::Up);
        let crossings_down = test_road.get_crossings(&Direction::Down);
        for idx in 0..crossings_up.len() {
            // Get reverse order idx
            let idx_rev = crossings_up.len() - 1 - idx;
            // Check Rc value same at reverse index
            assert!(&crossings_up[idx].0.eq(&crossings_down[idx_rev].0));
            // Check Rc reference same at reverse index
            assert!(Rc::ptr_eq(&crossings_up[idx].0, &crossings_down[idx_rev].0));
            // Check Rc value different at same index
            assert!(&crossings_up[idx].0.ne(&crossings_down[idx].0));
            // Check Rc reference different at same index
            assert!(!Rc::ptr_eq(&crossings_up[idx].0, &crossings_down[idx].0));
        }
    }

    #[test]
    fn test_get_crossing_position() {
        let crossings = vec![
	        (Crossing::Zebra { id: 0, cross_time: TimeDelta::from_secs(25) }, 10.0),
	        (Crossing::Zebra { id: 1, cross_time: TimeDelta::from_secs(10) }, 13.0),
	    ];
        let road = Road::new(30.0f32, crossings);

        assert_eq!(road.get_crossing_position(&0, Direction::Up), 10.0);
        assert_eq!(road.get_crossing_position(&1, Direction::Up), 13.0);

        assert_eq!(road.get_crossing_position(&0, Direction::Down), 30.0 - 10.0);
        assert_eq!(road.get_crossing_position(&1, Direction::Down), 30.0 - 13.0);
    }
}
