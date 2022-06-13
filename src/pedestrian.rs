use std::time::{Duration, Instant};

// use main::Crossing;

// mod main;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Crossing {
    Zebra(f32),
    Pelican
}

pub trait Person {
    fn location(&self) -> Crossing;
    fn arrival_time(&self) -> Instant;
}


pub struct Pedestrian { 
    location: Crossing,
    arrival_time: Instant,
    // timestamps: Vec <Instant>,
}

impl Person for Pedestrian {
    // Sample location
    // fn assign_location(&mut self) {
    // 
    // }
    
    fn location(&self) -> Crossing {
	// Crossing::Zebra(0.0f32)
	self.location
    }
    
    fn arrival_time(&self) -> Instant {
    // fn arrival_time(&self) -> Duration {
	// Duration::new(1, 0)
	self.arrival_time
    }

    // fn depart_time(&self) -> Instant {
    // // fn arrival_time(&self) -> Duration {
    // 	// Duration::new(1, 0)
    // 	Instant::now()
    // }

}

// How long does a person take to cross the road?
// 1. Person spawns at crossing at `arrival_time` e.g. t = t_a
// 2. Wait or immediately start crossing
// 3. Crossed after time t = t_a + t_c
//
// Actions:
// A. Car must consider person on road between time:
//    t_a <= t < t_a + t_c


#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    
    #[test]
    fn test_pedestrian_location(){
        let test_pedestrian = Pedestrian {
	    location: Crossing::Pelican,
	    arrival_time: Instant::now()
	};
		
        assert_eq!(test_pedestrian.location(), Crossing::Pelican);
    }

    #[test]
    fn test_pedestrian_zebra(){
        let test_pedestrian = Pedestrian {
	    location: Crossing::Zebra(0f32),
	    arrival_time: Instant::now()
	};
		
        assert_eq!(test_pedestrian.location(), Crossing::Zebra(0f32));
    }


    #[test]
    fn test_pedestrian_arrival(){
        let test_pedestrian = Pedestrian {
	    location: Crossing::Pelican,
	    arrival_time: Instant::now()
	};
	
	sleep(Duration::new(0, 10));
	let new_now = Instant::now();
        assert_ne!(test_pedestrian.arrival_time(), new_now);
    }

}
