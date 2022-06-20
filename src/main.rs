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

    let crossings = vec![
	        (Crossing::Zebra { id: 0, cross_time: TimeDelta::from_secs(10) }, 170.0),
	        (Crossing::Zebra { id: 1, cross_time: TimeDelta::from_secs(10) }, 290.0),
	    ];

    let road = Road::new(400.0f32, crossings);

    // TODO: debug why this isn't loading correcly
    // let road = Road::config_new();

    println!("{:?}", road.get_length());

    // let state = Box::new(SimulatorState::new());
    println!("{:?}, {:?}", &config.zebra_crossings, &config.road_length);
    
    let mut simulation = EventDrivenSim::new(
        0,0, 60_000,
        1., 1.,
        Box::new(SimulatorState::new()), 
        road
    );

    simulation.run();

    let as_json= to_json(&simulation.state).unwrap();
    println!("{}", &as_json);
}
