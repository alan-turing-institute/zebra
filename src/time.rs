
use std::ops::{Add, Mul};
use std::convert::Into;
use serde::{Serialize, Deserialize};


use crate::{Time, Length, Speed}; // , Acceleration};


#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Clone, Copy)]
pub struct TimeDelta(Time);

pub const TIME_RESOLUTION: Time = 1000;

impl TimeDelta {

    pub const fn new(millis: Time) -> TimeDelta { TimeDelta(millis) }
    pub const fn from_secs(secs: Time) -> TimeDelta { TimeDelta(secs * TIME_RESOLUTION) }
}

impl Into<f32> for &TimeDelta {
    fn into(self) -> f32 {
        self.0 as f32 / TIME_RESOLUTION as f32
    }
}

impl Into<f32> for TimeDelta {
    fn into(self) -> f32 {
        Into::<f32>::into(&self)
    }
}

impl From<i32> for TimeDelta {
    fn from(arg: i32) -> Self {
         TimeDelta(arg as Time)
    }
}

impl From<i64> for TimeDelta {
    fn from(arg: i64) -> Self {
         TimeDelta(arg as Time)
    }
}

impl From<f32> for TimeDelta {
    fn from(secs: f32) -> Self {
        TimeDelta(f32::round(secs * (TIME_RESOLUTION as f32)) as Time)
    }
}

impl From<&f32> for TimeDelta {
    fn from(arg: &f32) -> Self {
        From::<f32>::from(*arg)
    }
}

impl Add<Time> for &'_ TimeDelta {
    type Output = Time;

    fn add(self, rhs: Time) -> Self::Output {
        rhs + self.0
    }
}

impl Add<Time> for TimeDelta {
    type Output = Time;

    fn add(self, rhs: Time) -> Self::Output {
        rhs + self.0
    }
}

impl Mul<Speed> for &'_ TimeDelta {
    type Output = Length;

    fn mul(self, rhs: Speed) -> Self::Output {
        rhs * Into::<f32>::into(self)
    }
}

impl Add<TimeDelta> for Time {
    type Output = Time;

    fn add(self, rhs: TimeDelta) -> Self::Output {
        self + rhs.0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_convert_milis_to_float() {
        let t = TimeDelta::new(500);
        let fl: f32 = t.into();
        assert_eq!(fl, 0.500);
    }

    #[test]
    fn test_from_secs() {
        let t = TimeDelta::from_secs(5);
        let fl: f32 = t.into();
        assert_eq!(fl, 5.0);
    }

    #[test]
    fn test_from_float_roundtrip() {
        let t = TimeDelta::from(3.5);
        let TimeDelta(millis) = t;
        assert_eq!(millis, 3500);
    }

    #[test]
    fn test_add_to_time() {
        let t = TimeDelta::from_secs(15);
        let st: Time = 10_000i64;
        let result = t + st;

        assert_eq!(result, 25_000);
    }

}
