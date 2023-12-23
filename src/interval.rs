#[derive(Clone, Copy)]
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

    pub fn clamp(self, x: f64) -> f64 {
        if x <= self.lower {
            return self.lower;
        }
        if x >= self.upper {
            return self.upper;
        } else {
            return x;
        }
    }
}
