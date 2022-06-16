use zebra::*;


use clap::Parser;
use zebra::event_driven_sim::EventDrivenSim;

#[derive(Debug, Parser)]
struct CLIOptions {

}


fn main(){
    let args = CLIOptions::parse();

    let config = get_zebra_config();

//     seed: u64,
//     start_time: Time,
//     end_time: Time,
//     ped_arrival_rate: f32,
//     veh_arrival_rate: f32,
// // crossing_weights: Vec<f64>,
//     road: Road)

    let mut simulation = EventDrivenSim::new(0, 0, 60_000, 1., 1., Road::new(1000.0, Vec::new()));


}
