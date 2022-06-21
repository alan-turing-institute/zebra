use crate::{Time, ID};
use crate::road::{Crossing, CROSSING_TIME};
use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde_json::to_string as to_json;
use std::rc::Rc;

pub trait Person {
    fn set_id(&mut self, id: ID);
    fn get_id(&self) -> ID;
    fn location(&self) -> Crossing;
    fn arrival_time(&self) -> Time;
}

#[derive(Clone)]
pub struct Pedestrian {
    id: ID,
    location: Rc<Crossing>,
    arrival_time: Time,
}

impl Person for Pedestrian {
    fn location(&self) -> Crossing {
        *self.location
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
        assert_eq!(test_pedestrian.location(), *test_pelican);
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
    fn test_pedestrian_rc_equivalance() {
        let test_zebra = Rc::new(Crossing::zebra(0));
        let arrival_time = 0;
        let test_pedestrian = Pedestrian::new(0, Rc::clone(&test_zebra), arrival_time);

        // Expect the reference to crossing to be the same in pedestrian as test_zebra
        assert!(test_pedestrian.location.eq(&test_zebra));
    }
}
