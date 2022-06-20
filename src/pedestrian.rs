use crate::{Time, ID};
use crate::road::{Crossing, CROSSING_TIME};
use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde_json::to_string as to_json;

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

use std::borrow::Borrow;
use std::rc::Rc;

pub trait Person {
    fn set_id(&mut self, id: ID);
    fn get_id(&self) -> ID;
    fn location(&self) -> &Crossing;
    fn arrival_time(&self) -> Time;
}

#[derive(Clone)]
pub struct Pedestrian {
    id: ID,
    location: Rc<Crossing>,
    arrival_time: Time,
}

impl Person for Pedestrian {
    fn location(&self) -> &Crossing {
        self.location.borrow()
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
    pub fn new(id: ID, crossing: &Crossing, arrival_time: Time) -> Pedestrian {
        let location = Rc::new(*crossing);
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
        let test_pelican = Crossing::pelican(0);
        let test_pedestrian = Pedestrian::new(1, &test_pelican, 0);
        let as_json = to_json(&test_pedestrian).unwrap();
        assert_eq!(&as_json, "{\"id\":1,\"location\":0,\"arrival_time\":0}");
    }

    #[test]
    fn test_get_pedestrian_id() {
        let test_pelican = Crossing::pelican(0);
        let test_pedestrian = Pedestrian::new(1, &test_pelican, 0);
        assert_eq!(test_pedestrian.get_id(), 1);
    }

    #[test]
    fn test_pedestrian_location() {
        let test_pelican = Rc::new(Crossing::pelican(0));
        let test_pedestrian = Pedestrian::new(0, test_pelican.borrow(), 0);
        assert_eq!(test_pedestrian.location(), test_pelican.borrow());
    }

    #[test]
    fn test_pedestrian_arrival() {
        let test_zebra = Crossing::zebra(0);
        let arrival_time = 0;
        let test_pedestrian = Pedestrian::new(0, &test_zebra, arrival_time);
        let exit_time = test_zebra.stop_time() + test_pedestrian.arrival_time;

        // Expect the exit time to be the arrival time plus the time taken to cross.
        assert_eq!(exit_time, CROSSING_TIME + arrival_time);
    }
}
