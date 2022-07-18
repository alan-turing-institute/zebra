use zebra::*;

use clap::Parser;
use zebra::event_driven_sim::EventDrivenSim;
use zebra::state::SimulatorState;
use serde_json::to_string as to_json;
use clap::{arg, Arg, command, ArgAction};

// #[derive(Debug, Parser)]
// struct CLIOptions {}

fn main() {
    let matches = command!()
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::SetTrue),
        )
        .get_matches();    

    // Get configs
    let zebra_config = get_zebra_config();

    // Load road from config
    let road = Road::config_new();

    // Make simulation
    let mut simulation = EventDrivenSim::new(
        0,
        0,
        zebra_config.simulation.run_time,
        zebra_config.simulation.pedestrian_arrival_rate,
        zebra_config.simulation.vehicle_arrival_rate,
        Box::new(SimulatorState::new()), 
        road,
        *matches.get_one::<bool>("verbose").expect("defaulted by clap")
    );

    // Run simulation
    simulation.run();
}
