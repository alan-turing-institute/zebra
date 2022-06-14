
use std::convert::Into;
use serde::{Serialize, Deserialize};


#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TimeDelta(i64);

const TIME_RESOLUTION: i64 = 1000;

impl TimeDelta {

    pub fn new(millis: i64) -> TimeDelta { TimeDelta(millis) }
    pub fn from_secs(secs: i64) -> TimeDelta { TimeDelta(secs * TIME_RESOLUTION) }
}

impl Into<f32> for &TimeDelta {
    fn into(self) -> f32 {
        self.0 as f32 / TIME_RESOLUTION as f32
    }
}

impl From<i32> for TimeDelta {
    fn from(arg: i32) -> Self {
         TimeDelta(arg as i64)
    }
}

impl From<f32> for TimeDelta {
    fn from(secs: f32) -> Self {
        TimeDelta(f32::round(secs * (TIME_RESOLUTION as f32)) as i64)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_convert_milis_to_float() {
        let t = TimeDelta::new(100);
        let fl: f32 = t.into();

    }

}
