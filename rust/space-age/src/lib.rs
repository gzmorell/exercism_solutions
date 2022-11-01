// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.
const EARTH_YEAR_IN_SECONDS: f64 = 31557600.0;
const MERCURY_EARTH_YEARS: f64 = 0.2408467;
const VENUS_EARTH_YEARS: f64 = 0.61519726;
const MARS_EARTH_YEARS: f64 = 1.8808158;
const JUPITER_EARTH_YEARS: f64 = 11.862615;
const SATURN_EARTH_YEARS: f64 = 29.447498;
const URANUS_EARTH_YEARS: f64 = 84.016846;
const NEPTUNE_EARTH_YEARS: f64 = 164.79132;

#[derive(Debug)]
pub struct Duration {
    seconds: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s as f64 }
    }
}

pub trait Planet {
    fn earth_years() -> f64 {
        1.0
    }

    fn years_during(d: &Duration) -> f64 {
        d.seconds / EARTH_YEAR_IN_SECONDS / Self::earth_years()
    }
}

macro_rules! planets {
    ( $(($i:ident$(, $v:expr)?)),* ) => {
            $(
                pub struct $i;
                impl Planet for $i {
                    $(
                        fn earth_years() -> f64 {
                        $v
                    }
                    )?
                }
            )*
    };
}

planets![
    (Mercury, MERCURY_EARTH_YEARS),
    (Venus, VENUS_EARTH_YEARS),
    (Earth),
    (Mars, MARS_EARTH_YEARS),
    (Jupiter, JUPITER_EARTH_YEARS),
    (Saturn, SATURN_EARTH_YEARS),
    (Uranus, URANUS_EARTH_YEARS),
    (Neptune, NEPTUNE_EARTH_YEARS)
];
