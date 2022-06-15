use crate::road::Crossing;
use crate::time::TimeDelta;
use crate::Time;

use std::fs;
use std::collections::HashMap;
use std::default::Default;

use clap::lazy_static;
use toml;

use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;


// Type declarations, to make it easier to change later
pub type ArrivalRate = f32;
pub type Length = f32;
pub type Speed = f32;
pub type Acceleration = f32;

lazy_static!(
    static ref ZEBRA_CONFIG: ZebraConfig = fs::read("zebra.toml")
                            .ok()
                            .and_then(|data| toml::from_slice(&data).ok())
                            .unwrap_or_default();
);


/// Configuration settings specific to the simulation
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SimulationConfig {
    /// Total simulation time
    pub run_time: Time,
    /// Max number of pedestrians (active at once?)
    pub num_pedestrians: usize,
    /// Max number of vehicles (active at once?)
    pub num_vehicles: usize,
    /// Arrival rate for pedestrians - parameter in exponential distribution
    pub pedestrian_arrival_rate: ArrivalRate,
    /// Arrival rate for vehicles
    pub vehicle_arrival_rate: ArrivalRate
}


impl Default for SimulationConfig {
    fn default() -> Self {
        SimulationConfig {
            run_time: 300_000, // 5 minutes
            num_pedestrians: 10,
            num_vehicles: 10,
            pedestrian_arrival_rate: 0.5,
            vehicle_arrival_rate: 0.5
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ZebraConfig {
    /// Max road speed ~ 13.4m/s
    pub max_speed: Speed,
    /// The maximum acceleration value (the value that it is constantly when accelerating)
    pub max_acceleration: Acceleration,
    /// Maximum deceleration
    pub max_deceleration: Acceleration,
    /// Time for pedestrian to cross the road
    pub crossing_time: TimeDelta,
    /// Waiting time from arrival to change of pelican crossing light
    pub pelican_wait_time: TimeDelta,
    /// Minimum time after a change to green before another stop can happen
    pub pelican_go_time: TimeDelta,

    /// The simulation specific config
    pub simulation: SimulationConfig,
    // could also use Vec<SimulationConfig> if we wanted to run multiple simulations in one execution?

    /// Length of the road
    pub road_length: Length,
    /// Definition of crossings
    pub crossings: Vec<Crossing>,

    // A "catch all" for any keys that we don't define explicitly.
    #[serde(flatten)]
    pub other: HashMap<String, String>
}


impl Default for ZebraConfig {
    fn default() -> Self {
        ZebraConfig {
            max_speed: 13.41,
            max_acceleration: 4.0,
            max_deceleration: 3.0,
            crossing_time: TimeDelta::from_secs(8),
            pelican_wait_time: TimeDelta::from_secs(5),
            pelican_go_time: TimeDelta::from_secs(5),
            simulation: Default::default(),
            road_length: 1000.0,
            crossings: Vec::new(),
            other: HashMap::new()
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_simulation_config_empty() {
        let config_string = b"";
        let config: SimulationConfig = toml::from_slice(config_string).unwrap_or_default();

        assert_eq!(config, SimulationConfig::default())
    }

    #[test]
    fn test_deserialize_simulation_config_all_keys() {
        let config_string = br#"
        run_time = 600_000
        num_pedestrians = 500
        num_vehicles = 500
        pedestrian_arrival_rate = 5
        vehicle_arrival_rate = 5
        "#;
        let config: SimulationConfig = toml::from_slice(config_string).unwrap();

        assert_eq!(config.run_time, 600_000);
        assert_eq!(config.num_pedestrians, 500);
        assert_eq!(config.num_vehicles, 500);
        assert_eq!(config.pedestrian_arrival_rate, 5.0);
        assert_eq!(config.vehicle_arrival_rate, 5.0);
    }



}
