pub struct Interval {
    pub lower: f64,
    pub upper: f64,
}

impl Default for Interval {
    fn default() -> Self {
        Interval {
            lower: f64::NEG_INFINITY,
            upper: f64::INFINITY,
        }
    }
}

impl Interval {
    pub fn new(lower: f64, upper: f64) -> Self {
        Interval { lower, upper }
    }

    pub fn contains(&self, x: f64) -> bool {
        self.lower <= x && x <= self.upper
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.lower < x && x < self.upper
    }
}

const EMPTY: Interval = Interval {
    lower: f64::INFINITY,
    upper: f64::NEG_INFINITY,
};
const UNIVERSE: Interval = Interval {
    lower: f64::NEG_INFINITY,
    upper: f64::INFINITY,
};
