use zebra::*;


use clap::Parser;


#[derive(Debug, Parser)]
struct CLIOptions {

}


fn main(){
    let args = CLIOptions::parse();

    let config = get_zebra_config();

    let mut simulation = Simulation::new(0, 0, 0, 0.0, 0.0, Road::new(100.0, Vec::new()));


}
