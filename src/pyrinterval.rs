use ivtools::IntervalLike;
use pyo3::prelude::*;
use std::cmp::Ordering::{self};
/// Represent a range from [start, stop)
/// Inclusive start, exclusive of stop
#[pyclass]
#[derive(Debug)]
pub struct PyrInterval {
    #[pyo3(get, set)]
    pub start: u32,
    #[pyo3(get, set)]
    pub stop: u32,
    #[pyo3(get, set)]
    pub val: u32,
}

#[pymethods]
impl PyrInterval {
    #[new]
    fn __new(start: u32, stop: u32, val: u32) -> Self {
        PyrInterval {
            start: start,
            stop: stop,
            val: val,
        }
    }
}

impl IntervalLike<u32> for PyrInterval {
    #[inline]
    fn new(start: u32, stop: u32, val: u32) -> Self {
        PyrInterval { start, stop, val }
    }
    /// Compute the intsect between two intervals
    #[inline]
    fn intersect(&self, other: &PyrInterval) -> u32 {
        std::cmp::min(self.stop, other.stop)
            .checked_sub(std::cmp::max(self.start, other.start))
            .unwrap_or(0)
    }

    /// Check if two intervals overlap
    #[inline]
    fn overlap(&self, start: u32, stop: u32) -> bool {
        self.start < stop && self.stop > start
    }

    #[inline]
    fn start(&self) -> u32 {
        self.start
    }

    #[inline]
    fn stop(&self) -> u32 {
        self.stop
    }

    #[inline]
    fn val(&self) -> &u32 {
        &self.val
    }

    #[inline]
    fn set_start(&mut self, new: u32) {
        self.start = new;
    }

    #[inline]
    fn set_stop(&mut self, new: u32) {
        self.stop = new;
    }

    #[inline]
    fn set_val(&mut self, new: u32) {
        self.val = new;
    }
}

impl Ord for PyrInterval {
    #[inline]
    fn cmp(&self, other: &PyrInterval) -> Ordering {
        if self.start < other.start {
            Ordering::Less
        } else if other.start < self.start {
            Ordering::Greater
        } else {
            self.stop.cmp(&other.stop)
        }
    }
}
impl Eq for PyrInterval {}

impl PartialOrd for PyrInterval {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for PyrInterval {
    #[inline]
    fn eq(&self, other: &PyrInterval) -> bool {
        self.start == other.start && self.stop == other.stop
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod test {
    use super::*;
    type Iv = PyrInterval;
    #[test]
    fn test_interval_intersects() {
        let i1 = Iv{start: 70, stop: 120, val: 0}; // max_len = 50
        let i2 = Iv{start: 10, stop: 15, val: 0};
        let i3 = Iv{start: 10, stop: 15, val: 0}; // exact overlap
        let i4 = Iv{start: 12, stop: 15, val: 0}; // inner overlap
        let i5 = Iv{start: 14, stop: 16, val: 0}; // overlap end
        let i6 = Iv{start: 40, stop: 50, val: 0};
        let i7 = Iv{start: 50, stop: 55, val: 0};
        let i_8 = Iv{start: 60, stop: 65, val: 0};
        let i9 = Iv{start: 68, stop: 71, val: 0}; // overlap start
        let i10 = Iv{start: 70, stop: 75, val: 0};

        assert_eq!(i2.intersect(&i3), 5); // exact match
        assert_eq!(i2.intersect(&i4), 3); // inner intersect
        assert_eq!(i2.intersect(&i5), 1); // end intersect
        assert_eq!(i9.intersect(&i10), 1); // start intersect
        assert_eq!(i7.intersect(&i_8), 0); // no intersect
        assert_eq!(i6.intersect(&i7), 0); // no intersect stop = start
        assert_eq!(i1.intersect(&i10), 5); // inner intersect at start
    }
}
