use crate::pedestrian::Person;
use crate::{Crossing, Time};
use crate::vehicle::{Vehicle};

use std::cmp::{Ord, Eq, Ordering};

#[derive(Debug)]
#[non_exhaustive]
pub enum EventType<'a> {
    VehicleArrival,
    VehicleExit(&'a dyn Vehicle),
    SpeedLimitReached(&'a dyn Vehicle),
    ZeroSpeedReached(&'a dyn Vehicle),
    ReactionToObstacle(&'a dyn Vehicle),

    PedestrianArrival,
    PedestrianExit(&'a dyn Person),

    LightsToRed(&'a Crossing),
    LightsToGreen(&'a Crossing),

}


pub enum EventResult<'a> {
    VehicleChange(&'a dyn Vehicle),
    PedestrianChange(&'a dyn Person),
    NoEffect
}


pub struct Event<'a>(pub Time, pub EventType<'a>);

impl<'a> PartialEq for Event<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<'a> Eq for Event<'a> {}

impl<'a> PartialOrd for Event<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Ordering::*;
        if self.0 > other.0 {
            Some(Greater)
        } else if self.0 < other.0 {
            Some(Less)
        } else {
            Some(Equal)
        }
    }
}

impl<'a> Ord for Event<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        use Ordering::*;
        if self.0 > other.0 {
            Greater
        } else if self.0 < other.0 {
            Less
        } else {
            Equal
        }
    }
}
