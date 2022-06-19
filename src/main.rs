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

    let road = Road::config_new();

    // let state = Box::new(SimulatorState::new());
    
    let mut simulation = EventDrivenSim::new(
        0,0, 60_000,
        1., 1.,
        Box::new(SimulatorState::new()), 
        Road::config_new()
    );

    simulation.run();

    let as_json= to_json(&simulation.state).unwrap();
    println!("{}", &as_json);
}
