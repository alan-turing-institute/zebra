use crate::obstacle::Obstacle;
use crate::{Time, ID};
use crate::road::{Crossing, CROSSING_TIME, Direction, Road};
use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde_json::to_string as to_json;
use std::rc::Rc;

pub trait Person {
    fn set_id(&mut self, id: ID);
    fn get_id(&self) -> ID;
    fn location(&self) -> &Rc<Crossing>;
    fn arrival_time(&self) -> Time;
}

#[derive(Debug)]
pub struct Pedestrian {
    id: ID,
    location: Rc<Crossing>,
    arrival_time: Time,
}

impl Person for Pedestrian {
    fn location(&self) -> &Rc<Crossing> {
        &self.location
    }

    fn arrival_time(&self) -> Time {
        self.arrival_time
    }

    fn set_id(&mut self, id: ID) {
	self.id = id;
    }

    fn get_id(&self) -> ID {
        self.id
    }
}

impl Obstacle for Pedestrian {
    fn get_position(&self, road: &Road, direction: &Direction) -> f32 {
        // Check that the direction matches my direction.
        // let my_direction = &self.get_direction();
        // assert!(matches!(direction, my_direction));
        // self.location
        let id = self.location().get_id();
        road.get_crossing_position(&id, *direction)
    }

    fn get_obstacle_length(&self) -> f32 {
        0.0
    }

    fn get_speed(&self) -> f32 {
        0.0
    }

    fn get_acceleration(&self) -> f32 {
        0.0
    }

    fn is_active(&self, time: Time) -> bool {
        let arrival_time = self.arrival_time();
        let crossing_time = self.location.stop_time();
        let end_time = crossing_time + arrival_time;
        if time < end_time && time >= arrival_time {
            return true;
        }
        false
    }
}


impl Pedestrian {
    pub fn new(id: ID, location: Rc<Crossing>, arrival_time: Time) -> Pedestrian {
        Pedestrian {
            id,
            location,
            arrival_time,
        }
    }
}

impl Serialize for Pedestrian {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Number of fields in the struct and name.
        let mut state = serializer.serialize_struct("Pedestrian", 3)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("location", &self.location.get_id())?;
        state.serialize_field("arrival_time", &self.arrival_time)?;
        state.end()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_pedestrian() {
        let test_pelican = Rc::new(Crossing::pelican(0));
        let test_pedestrian = Pedestrian::new(1, Rc::clone(&test_pelican), 0);
        let as_json = to_json(&test_pedestrian).unwrap();
        assert_eq!(&as_json, "{\"id\":1,\"location\":0,\"arrival_time\":0}");
    }

    #[test]
    fn test_get_pedestrian_id() {
        let test_pelican = Rc::new(Crossing::pelican(0));
        let test_pedestrian = Pedestrian::new(1, Rc::clone(&test_pelican), 0);
        assert_eq!(test_pedestrian.get_id(), 1);
    }

    #[test]
    fn test_pedestrian_location() {
        let test_pelican = Rc::new(Crossing::pelican(0));
        let test_pedestrian = Pedestrian::new(0, Rc::clone(&test_pelican), 0);
        assert_eq!(*test_pedestrian.location(), test_pelican);
    }

    #[test]
    fn test_pedestrian_arrival() {
        let test_zebra = Rc::new(Crossing::zebra(0));
        let arrival_time = 0;
        let test_pedestrian = Pedestrian::new(0, Rc::clone(&test_zebra), arrival_time);
        let exit_time = test_zebra.stop_time() + test_pedestrian.arrival_time;

        // Expect the exit time to be the arrival time plus the time taken to cross.
        assert_eq!(exit_time, CROSSING_TIME + arrival_time);
    }

    #[test]
    fn test_pedestrian_is_active() {
        let test_zebra = Rc::new(Crossing::zebra(0));
        let arrival_time = 2000;
        let test_pedestrian = Pedestrian::new(0, Rc::clone(&test_zebra), arrival_time);

        // Expect the exit time to be the arrival time plus the time taken to cross.
        assert_eq!(test_pedestrian.is_active(arrival_time - 1), false);
        assert_eq!(test_pedestrian.is_active(arrival_time), true);
        assert_eq!(test_pedestrian.is_active(arrival_time + test_zebra.stop_time() - 1), true);
        assert_eq!(test_pedestrian.is_active(arrival_time + test_zebra.stop_time()), false);
    }

    #[test]
    fn test_pedestrian_rc_equivalance() {
        let test_zeb1 = Rc::new(Crossing::zebra(0));
        let test_zeb2 = Rc::new(Crossing::zebra(1));
        let arrival_time = 0;
        let test_ped1 = Pedestrian::new(0, Rc::clone(&test_zeb1), arrival_time);
        let test_ped2 = Pedestrian::new(0, Rc::clone(&test_zeb2), arrival_time);

        // Check same values
        assert!(test_ped1.location().eq(&test_zeb1));
        assert!(test_ped2.location().eq(&test_zeb2));

        // Check same references
        assert!(Rc::ptr_eq(&test_ped1.location, &test_zeb1));
        assert!(Rc::ptr_eq(&test_ped2.location, &test_zeb2));

        // Check difference references
        assert!(!Rc::ptr_eq(&test_ped1.location, &test_ped2.location));
    }
}
