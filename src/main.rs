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
        .arg(
            arg!(-o --outfile <OUTFILE>)
            .required(false)
        )
        .arg(
            arg!(-c --config_file <CONFIG_FILE>)
            .required(false)
        )
        .get_matches();    

    // Get configs
    // let zebra_config = get_zebra_config();
    let zebra_config = get_zebra_config_option(matches.get_one::<String>("config_file"));

    // Load road from config
    let road = Road::config_new(matches.get_one::<String>("config_file"));

    // Extract outfile arg as Option<String>
    let outfile = match matches.get_one::<String>("outfile") {
        Some(x) => Some(x.clone()),
        _ => None
    };

    // Make simulation
    let mut simulation = EventDrivenSim::new(
        0,
        0,
        zebra_config.simulation.run_time,
        zebra_config.simulation.pedestrian_arrival_rate,
        zebra_config.simulation.vehicle_arrival_rate,
        Box::new(SimulatorState::new()), 
        road,
        outfile,
        *matches.get_one::<bool>("verbose").expect("defaulted by clap")
    );

    // Run simulation
    simulation.run();
}
