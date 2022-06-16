use crate::pedestrian::Person;
use crate::{Crossing, Time};
use crate::vehicle::{Vehicle};

use std::cmp::{Ord, Eq, Ordering};

#[derive(Debug)]
#[non_exhaustive]
pub enum EventType<'a> {
    VehicleArrival,
    VehicleExit(usize),
    SpeedLimitReached(usize),
    ZeroSpeedReached(usize),
    ReactionToObstacle(usize),

    PedestrianArrival,
    PedestrianExit(usize),

    LightsToRed(usize),
    LightsToGreen(usize),

    StopSimulation
}


pub enum EventResult<'a> {
    NewVehicle(&'a dyn Vehicle),
    RemoveVehicle,
    VehicleChange(&'a dyn Vehicle),
    NewPedestrian(&'a dyn Person),
    RemovePedestrian,
    PedestrianChange(&'a dyn Person),
    CrossingChange(&'a Crossing),
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
