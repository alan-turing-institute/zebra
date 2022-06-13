use std::time::Instant;

// use main::Crossing;

// mod main;

#[derive(PartialEq, Debug)]
enum Crossing {
    Zebra(f32),
    Pelican
}

trait Person {
    fn location(&self) -> Crossing;
    fn arrival_time(&self) -> Instant;
}


struct Pedestrian { 
    location: Crossing,
    arrival_time: Instant
    
}

impl Person for Pedestrian {
    fn location(&self) -> Crossing {
	// Crossing::Zebra(0.0f32)
	Crossing::Pelican
    }
    fn arrival_time(&self) -> Instant {
	Instant::now()
    }
}

// How long does a person take to cross the road?

// 1. Person spawns at crossing t = t_a
// 2. Wait or immediately start crossing
// 3. Crossed after time t = t_a + t_c
//
// Actions:
// A. Car must consider person on road between time:
//    t_a <= t < t_a + t_c


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pedestrian_location(){
        let test_pedestrian = Pedestrian {
	    location: Crossing::Pelican,
	    arrival_time: Instant::now()
	};
		
        assert_eq!(test_pedestrian.location(), Crossing::Pelican);
    }

    // #[test]
    // fn test_pedestrian_arrival(){
    //     let test_pedestrian = Pedestrian {
    // 	    location: Crossing::Pelican,
    // 	    arrival_time: Instant::now()
    // 	};
		
    //     assert_eq!(test_pedestrian.location(), Crossing::Pelican);
    // }

}
