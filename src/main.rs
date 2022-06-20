use zebra::*;

use clap::Parser;
use zebra::event_driven_sim::EventDrivenSim;
use zebra::state::SimulatorState;
use serde_json::to_string as to_json;

#[derive(Debug, Parser)]
struct CLIOptions {}

fn main() {
    let args = CLIOptions::parse();

    let config = get_zebra_config();

    // let road = Road::config_new();
    let crossings = vec![
	        (Crossing::Zebra { id: 0, cross_time: TimeDelta::from_secs(10) }, 170.0),
	        (Crossing::Zebra { id: 1, cross_time: TimeDelta::from_secs(10) }, 290.0),
	    ];

    let road = Road::new(300.0f32, crossings);

    // let state = Box::new(SimulatorState::new());
    println!("{:?}, {:?}", &config.zebra_crossings, &config.road_length);
    
    let mut simulation = EventDrivenSim::new(
        0,0, 60_000,
        1., 1.,
        Box::new(SimulatorState::new()), 
        // Road::config_new()
        road
    );

    simulation.run();

    let as_json= to_json(&simulation.state).unwrap();
    println!("{}", &as_json);
}
