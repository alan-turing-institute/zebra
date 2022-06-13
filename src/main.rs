use std::time::Instant;


fn main(){
    print!("Hello world");
}

enum Action {
    Accelerate,
    Deccelerate,
    StaticSpeed
}


enum Crossing {
    Zebra(f32),
    Pelican
}


trait Vehicle {
    fn length() -> f32;
    fn position(&self) -> f32;
    // or (f32, f32)
    fn speed(&self) -> f32;
    fn acceleration(&self) -> f32;
    fn action(&self) -> Action;
}

trait Pedestrian {

}

struct Car {
    position: f32
}



impl Vehicle for Car {
    
    fn length() -> f32 {
        1.0f32
    }
    
    fn position(&self) -> f32 {
        self.position
    }
    
    fn speed(&self) -> f32 {
        0.0f32
    }
    
    fn acceleration(&self) -> f32 {
        0.0f32
    }
    
    fn action(&self) -> Action {
        Action::Accelerate
    }
    
}


trait State {
    const ROAD_LENGTH: f32;
    const CROSSINGS: Vec<Crossing>;


    // fn get_vehicles(&self) -> &[dyn Vehicle];
    fn timestamp(&self) -> Instant;

    fn update(&mut self);


}


struct ZebraState {
    vehicles: Vec<Car>,
    pedestrians: Vec<Box<dyn Pedestrian>>
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_car_postion(){
        let test_car = Car{position: 0.0};
        assert_eq!(test_car.position(), 0.0);
    }
}

