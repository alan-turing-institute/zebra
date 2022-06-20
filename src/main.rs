use zebra::*;

use clap::Parser;
use zebra::event_driven_sim::EventDrivenSim;
use zebra::state::SimulatorState;
use serde_json::to_string as to_json;

#[derive(Debug, Parser)]
struct CLIOptions {}

fn main() {
    let args = CLIOptions::parse();

    // Load road from config
    let road = Road::config_new();

    // Make simulation
    let mut simulation = EventDrivenSim::new(
        0,0, 60_000,
        1., 1.,
        Box::new(SimulatorState::new()), 
        road
    );

    // Run simulation
    simulation.run();
}
