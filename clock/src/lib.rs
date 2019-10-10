use core::{fmt, cmp};

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut m = minutes % 60;
        let mut h = minutes / 60;

        if m < 0 {
            m += 60;
            h -= 1;
        };

        h = (hours + h) % 24;
        if (h < 0) {
            h = h + 24;
        }
        Clock { hours: h, minutes: m }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let m = &self.minutes + minutes;
        let h = m / 60;
        let minutes = m % 60;
        let hours = (&self.hours + h) % 24;
        Clock::new(hours, minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl cmp::PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}
