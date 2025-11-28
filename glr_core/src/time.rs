use std::ops::{Add, AddAssign, Sub, SubAssign};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Serialize, Deserialize)]
pub struct Time(u64);

impl Default for Time {
    fn default() -> Self {
        Self(0)
    }
}

impl Time {
    ///
    /// Creates a Time with stamp 0
    ///
    pub const fn new() -> Time {
        return Time(0);
    }

    pub const fn from_stamp(stamp: u64) -> Time {
        return Time(stamp);
    }

    pub const fn from_min_secs(min: u64, sec: u64) -> Time {
        Time((min * 60 + sec) * 1000)
    }

    ///
    /// Creates a Time from a String
    /// String format: {Hours}:{Minutes}:{Seconds}.{Milliseconds}
    ///
    pub fn from(time: &str) -> Option<Time> {
        let hours: u64 = time.get(0..2)?.parse::<u64>().ok()?;
        let minutes: u64 = time.get(3..5)?.parse::<u64>().ok()?;
        let seconds: u64 = time.get(6..8)?.parse::<u64>().ok()?;
        let milliseconds: u64 = time.get(9..12)?.parse::<u64>().ok()?;

        Some(Time(
            (((hours * 60 + minutes) * 60) + seconds) * 1000 + milliseconds,
        ))
    }

    pub const fn max() -> Time {
        Time(24 * 60 * 60 * 1000)
    }

    pub fn get_stamp(&self) -> u64 {
        return self.0;
    }

    ///
    /// Returns a string showing the time of the object.
    ///
    pub fn to_string(&self) -> String {
        let milliseconds: u64 = self.0 % 1000;
        let seconds: u64 = self.0 / 1000 % 60;
        let minutes: u64 = self.0 / 60000 % 60;
        let hours: u64 = self.0 / 3600000;

        return format!(
            "{:02}:{:02}:{:02}.{:03}",
            hours, minutes, seconds, milliseconds
        );
    }

    ///
    /// Returns a string showing the time of the object
    /// without the hours mark if the hours is 0
    ///
    pub fn to_string_no_hours(&self) -> String {
        let milliseconds: u64 = self.0 % 1000;
        let seconds: u64 = self.0 / 1000 % 60;
        let minutes: u64 = self.0 / 60000 % 60;
        let hours: u64 = self.0 / 3600000;

        if hours > 0 {
            return format!(
                "{:02}:{:02}:{:02}.{:03}",
                hours, minutes, seconds, milliseconds
            );
        }

        format!("{:02}:{:02}.{:03}", minutes, seconds, milliseconds)
    }

    pub fn min(&self, other: &Time) -> Time {
        if self < other {
            return *self;
        }

        *other
    }
}

impl From<u64> for Time {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl Add for Time {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Time {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for Time {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self < rhs {
            return Time(self.0 + 24 * 60 * 60 * 1000 - rhs.0);
        }

        Time(self.0 - rhs.0)
    }
}

impl SubAssign for Time {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.sub(rhs);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let time: Time = Time::new();
        let stamp: u64 = time.0;

        assert_eq!(stamp, 0);
    }

    #[test]
    fn test_from() {
        let time1 = Time::from("11:22:33.444");
        let time2 = Time::from("23:59:59.999");
        let time3 = Time::from("00:00:00.000");

        assert_eq!(time1.unwrap().0, 40953444);
        assert_eq!(time2.unwrap().0, 86399999);
        assert_eq!(time3.unwrap().0, 0);
    }

    #[test]
    fn test_from_overflow() {
        let time1 = Time::from("11:22:33.44400");
        let time2 = Time::from("23:59:59.999adda");
        let time3 = Time::from("00:00:00.000111");

        assert_eq!(time1.unwrap().0, 40953444);
        assert_eq!(time2.unwrap().0, 86399999);
        assert_eq!(time3.unwrap().0, 0);
    }

    #[test]
    fn test_to_string() {
        let time1 = Time::from("11:22:33.444");
        let time2 = Time::from("23:59:59.999");
        let time3 = Time::from("00:00:00.000");

        assert_eq!(time1.unwrap().to_string(), "11:22:33.444");
        assert_eq!(time2.unwrap().to_string(), "23:59:59.999");
        assert_eq!(time3.unwrap().to_string(), "00:00:00.000");
    }

    #[test]
    fn test_add() {
        let time1 = Time::from("00:00:00.010").unwrap();
        let time2 = Time::from("00:00:00.133").unwrap();

        assert_eq!((time1 + time2).to_string(), "00:00:00.143".to_string());
    }

    #[test]
    fn test_sub_normal() {
        let time1: Time = Time::from("00:00:00.500").unwrap();
        let time2: Time = Time::from("00:00:00.133").unwrap();

        assert_eq!((time1 - time2).to_string(), "00:00:00.367".to_string());
    }

    #[test]
    fn test_sub_underflow() {
        let time1: Time = Time::from("00:00:00.500").unwrap();
        let time2: Time = Time::from("00:00:01.000").unwrap();

        assert_eq!((time1 - time2).to_string(), "23:59:59.500".to_string());
    }

    #[test]
    fn test_operators() {
        let time1: Time = Time::from("00:00:01.000").unwrap();
        let time2: Time = Time::from("00:00:02.000").unwrap();

        assert!(time2 > time1);
        assert!(time2 >= time1);
        assert!(time1 < time2);
        assert!(time1 <= time2);
        assert!(time1 >= time1);
        assert!(time2 >= time2);
    }
}
