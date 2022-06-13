use std::time::{Instant, Duration};


fn main(){
    print!("Hello world");
}

enum Action {
    Accelerate,
    Deccelerate,
    StaticSpeed
}


trait Vehicle {
    fn get_length() -> f32;
    fn get_buffer_zone() -> f32;
    fn get_position(&self) -> f32;
    // or (f32, f32)
    fn get_speed(&self) -> f32;
    fn get_acceleration(&self) -> f32;
    fn action(&mut self, action:Action);
    fn roll_forward_by(&mut self, duration: Duration);
}


struct Car {
    length: f32,
    buffer_zone: f32,
    position: f32,
    speed: f32,
    acceleration: f32,
    
}

// impl Car {
//     fn set_acceleration() -> 
// }

impl Vehicle for Car {
    
    fn get_length() -> f32 {
       self.length
    }
    fn get_buffer_zone() -> f32 {
        self.buffer_zone
    }
    fn get_position(&self) -> f32 {
        self.position
    }
    
    fn get_speed(&self) -> f32 {
        self.speed
    }
    
    fn get_acceleration(&self) -> f32 {
        self.acceleration
    }
    
    fn action(&mut self, action:Action) {
        match action {
        Action::Accelerate  => self.acceleration = 3,
        Action::Deccelerate => self.acceleration = -4,
        Action::StaticSpeed => self.acceleration = 0};
    }
    
    fn roll_forward_by(&mut self, duration: Duration) {
        let position = self.position;
        let speed = self.speed;
        let acceleration = self.acceleration;
    
        let seconds = duration.as_secs();
        
        speed = speed + acceleration * seconds;
        position = position + (0.5 * acceleration * seconds * seconds);

        assert!(speed < 13.41);
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_car_postion(){
        let test_car = Car{position: 0.0};
        assert_eq!(test_car.position(), 0.0);
    }

    #[test]
    fn test_car_length(){
        let test_car = Car{length: 4.0};
        assert_eq!(test_car.length(),4.0);
    }


}

