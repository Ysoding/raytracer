pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, val: f64) -> bool {
        self.min <= val && val <= self.max
    }

    pub fn surrounds(&self, val: f64) -> bool {
        self.min < val && val < self.max
    }

    pub fn empty() -> Self {
        Self {
            min: std::f64::INFINITY,
            max: std::f64::NEG_INFINITY,
        }
    }

    pub fn universe() -> Self {
        Self {
            min: std::f64::NEG_INFINITY,
            max: std::f64::INFINITY,
        }
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self::empty()
    }
}
