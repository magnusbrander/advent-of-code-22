pub mod interval {
    pub struct Interval {
        pub start: u32,
        pub stop: u32,
    }

    impl Interval {
        pub fn contains(&self, interval: &Interval) -> bool {
            return interval.start >= self.start && interval.stop <= self.stop;
        }

        pub fn overlapps(&self, interval: &Interval) -> bool {
            let range = self.start..(self.stop + 1);
            return range.contains(&interval.start) || range.contains(&interval.stop);
        }
    }
}

pub mod interval_pair {
    use super::interval::Interval;

    pub struct IntervalPair {
        pub first: Interval,
        pub second: Interval,
    }

    impl IntervalPair {
        pub fn is_any_contained(&self) -> bool {
            return self.first.contains(&self.second) || self.second.contains(&self.first);
        }

        pub fn overlapps(&self) -> bool {
            return self.first.overlapps(&self.second) || self.second.overlapps(&self.first);
        }
    }
}
