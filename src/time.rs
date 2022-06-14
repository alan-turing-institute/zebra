
use std::convert::Into;



#[repr(transparent)]
#[derive()]
pub struct TimeDelta(i64);

const TIME_RESOLUTION: i64 = 1000;

impl TimeDelta {

    pub fn new(millis: i64) -> TimeDelta { TimeDelta(time) }
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
        todo!()
    }
}
