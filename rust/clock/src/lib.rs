use std::fmt;

const HOURS_CYCLE: i32 = 24;
const MINUTES_CYCLE: i32 = 60;

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            hours: (hours + minutes.div_euclid(MINUTES_CYCLE)).rem_euclid(HOURS_CYCLE),
            minutes: minutes.rem_euclid(MINUTES_CYCLE),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self {
            hours: (self.hours + (self.minutes + minutes).div_euclid(MINUTES_CYCLE))
                .rem_euclid(HOURS_CYCLE),
            minutes: (self.minutes + minutes).rem_euclid(MINUTES_CYCLE),
        }
    }

    // pub fn to_string(&self) -> String {
    //     format!("{:02}:{:02}", self.hours, self.minutes)
    // }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours.rem_euclid(HOURS_CYCLE) == other.hours.rem_euclid(HOURS_CYCLE)
            && self.minutes.rem_euclid(MINUTES_CYCLE) == other.minutes.rem_euclid(MINUTES_CYCLE)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02} Hours, {:02} Minutes", self.hours, self.minutes)
    }
}

impl Copy for Clock {}

impl Clone for Clock {
    fn clone(&self) -> Self {
        *self
    }
}
