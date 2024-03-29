use crate::pedestrian::Person;
use crate::{ID, Crossing, Time};
use crate::vehicle::{Vehicle};

use std::cmp::{Ord, Eq, Ordering};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
#[non_exhaustive]
pub enum EventType {
    // Ordered according to the order they should be handled
    VehicleAccelerate(usize),
    SpeedLimitReached(usize),
    ZeroSpeedReached(usize),
    StaticSpeedReached(usize),
    ReactionToObstacle(usize),

    EmergencyStop(usize),

    LightsToRed(usize),
    LightsToGreen(usize),

    // Events placed last so can be sorted to handle these last
    VehicleExit(ID),
    PedestrianExit(ID),
    VehicleArrival,
    PedestrianArrival,

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

#[derive(Debug)]
pub struct Event(pub Time, pub EventType);

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for Event {}

impl PartialOrd for Event {
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

impl Ord for Event {
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
