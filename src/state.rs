use crate::pedestrian::Pedestrian;
use crate::Time;
use crate::time::TimeDelta;
use crate::vehicle::{Vehicle, Car, Action};
use crate::road::Direction;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde_json::to_string as to_json;

pub trait State {

    // fn get_vehicles(&self) -> &[dyn Vehicle];
    fn timestamp(&self) -> &Time;

    // get the list of vehicles
    fn get_vehicles(&self) -> &Vec<Box<dyn Vehicle>>;

    // get the list of pedestrians
    fn get_pedestrians(&self) ->  &Vec<Pedestrian>;

    // MOVED TO THE SIMULATION TRAIT:
    // // get time interval until next event
    // fn time_to_next_event(&self, ped_arrival_times: &[Time], veh_arrival_times: &[Time]) -> TimeDelta;

    // // roll state forward by time interval
    // fn roll_forward_by(&mut self, time_delta: TimeDelta);

    // // update state
    // fn instantaneous_update(&mut self);

}

pub struct SimulatorState<'a> {

    vehicles: Vec<Box<dyn Vehicle>>,
    pedestrians: Vec<Pedestrian<'a>>,
    timestamp: Time
}

impl Serialize for SimulatorState <'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Number of fields in the struct and name.
        let mut state = serializer.serialize_struct("State", 3)?;
        state.serialize_field("timestamp", &self.timestamp)?;
        state.serialize_field("pedestrians", &self.get_pedestrians())?;
        // TODO: implement Serialize for vehicle trait if possible
        // state.serialize_field("cars", &self.vehicles)?;
        state.end()
    }
}

// 1. New struct (SerializedState) to hold data at a given timestamp
// 2. SerializedState.read(SimulatorState) 
// 3. Annotate the owned data in SerializedState with #[derive(Serialize, Deserialize)]
// How does serde handle dynamic objects?
// #[derive(Serialize)]
// pub struct SerializedState <'a> {
//     // vehicles: Vec<Vehicle>,
//     pedestrians: Vec<Pedestrian<'a>>,

// }
// ----
// use serde::ser::{Serialize, Serializer, SerializeSeq, SerializeMap};
// use serde::ser::{Serializer, SerializeSeq, SerializeMap};
// impl Serialize for Vec<Pedestrian<'a>>
// // where
// //T: Serialize,
// {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut seq = serializer.serialize_seq(Some(self.len()))?;
//         for e in self {
//             seq.serialize_element(e)?;
//         }
//         seq.end()
//     }
// }

// ----

impl<'a> SimulatorState<'a> {

    // Constructor for the initial state at time 0.
    pub fn new() -> SimulatorState<'a> {

        SimulatorState {vehicles: Vec::new(), pedestrians: Vec::new(), timestamp: 0}
    }

    // Construct a state with arbitrary content
    pub fn dummy(vehicles: Vec<Box<dyn Vehicle>>,
        pedestrians: Vec<Pedestrian<'a>>,
        timestamp: Time) -> SimulatorState<'a> {

        SimulatorState{vehicles, pedestrians, timestamp}
    }
}

impl<'a> State for SimulatorState<'a> {

    fn timestamp(&self) -> &Time {
        &self.timestamp
    }

    // get the list of vehicles
    fn get_vehicles(&self) -> &Vec<Box<dyn Vehicle>> {
        &self.vehicles
    }

    // get the list of pedestrians
    fn get_pedestrians(&self) ->  &Vec<Pedestrian> {
        &self.pedestrians
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulator_state_constructor() {

        let state = SimulatorState::new();

        // assert_eq!(state.timestamp(), 0.0); // Initial timestamp is zero.

        assert_eq!(state.get_vehicles().len(), 0); // No vehicles
        assert_eq!(state.get_pedestrians().len(), 0); // No pedestrians
    }

}
