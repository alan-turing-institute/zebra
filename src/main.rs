use zebra::*;

use clap::Parser;
use zebra::event_driven_sim::EventDrivenSim;
use serde_json::to_string as to_json;

#[derive(Debug, Parser)]
struct CLIOptions {}

fn main() {
    let args = CLIOptions::parse();

    let config = get_zebra_config();

    // print!("{}\n", config.road_length);
    let road = Road::config_new();
    let cross = road.get_crossings(&Direction::Up);
    // print!("{}\n", cross.len());

    let mut simulation = EventDrivenSim::new(
        0, 0, 60_000,
        1., 1.,
        Road::config_new()
    );

    simulation.run();

    let as_json= to_json(&simulation.state).unwrap();
    println!("{}", &as_json);
}
