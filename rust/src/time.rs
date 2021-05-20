#[derive(Clone, Copy, Debug)]
pub struct TimeRange {
    pub lo: f64,
    pub hi: f64,
}

impl TimeRange {
    pub const ZERO: TimeRange = TimeRange { lo: 0.0, hi: 0.0 };

    pub fn new(lo: f64, hi: f64) -> Self {
        TimeRange { lo, hi }
    }

    pub fn len(self) -> f64 {
        self.hi - self.lo
    }

    pub fn contains(self, t: f64) -> bool {
        self.lo <= t && t <= self.hi
    }
}
