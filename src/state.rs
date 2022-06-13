
trait State {

    // const ROAD_LENGTH: f32;
    // const CROSSINGS: Vec<Crossing>;


    // fn get_vehicles(&self) -> &[dyn Vehicle];
    fn timestamp(&self) -> Instant;

    // get the road
    fn get_road(&self) -> Road;

    // get the list of vehicles
    fn get_vehicles(&self) -> Vec<Box<dyn Vehicle>>;

    // get the list of pedestrians
    fn get_pedestrians(&self) ->  Vec<Box<dyn Pedestrian>>;

    fn update(&mut self);


}


struct ZebraState {
    vehicles: Vec<Car>,
    pedestrians: Vec<Box<dyn Pedestrian>>

}
