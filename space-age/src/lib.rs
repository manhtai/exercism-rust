// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: f64
}

const ONE_EARTH_YEAR_IN_SECONDS: f64 = 31_557_600.0;

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s as f64 }
    }
}

pub trait Planet {
    const RATE: f64;
    fn years_during(d: &Duration) -> f64 {
        d.seconds / ONE_EARTH_YEAR_IN_SECONDS / Self::RATE
    }
}

pub struct Mercury;

pub struct Venus;

pub struct Earth;

pub struct Mars;

pub struct Jupiter;

pub struct Saturn;

pub struct Uranus;

pub struct Neptune;

impl Planet for Mercury {
    const RATE: f64 = 0.2408467;
}

impl Planet for Venus {
    const RATE: f64 = 0.61519726;
}

impl Planet for Earth {
    const RATE: f64 = 1.0;
}

impl Planet for Mars {
    const RATE: f64 = 1.8808158;
}

impl Planet for Jupiter {
    const RATE: f64 = 11.862615;
}

impl Planet for Saturn {
    const RATE: f64 = 29.447498;
}

impl Planet for Uranus {
    const RATE: f64 = 84.016846;
}

impl Planet for Neptune {
    const RATE: f64 = 164.79132;
}
