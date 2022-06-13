
trait State {

    // const ROAD_LENGTH: f32;
    // const CROSSINGS: Vec<Crossing>;


    // fn get_vehicles(&self) -> &[dyn Vehicle];
    fn timestamp(&self) -> Instant;

    fn update(&mut self);


}


struct ZebraState {
    vehicles: Vec<Car>,
    pedestrians: Vec<Box<dyn Pedestrian>>

}
