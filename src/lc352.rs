use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq)]
struct Interval {
    start: i32,
    end: i32,
}

impl Interval {
    fn new(start: i32, end: i32) -> Self {
        Self { start, end }
    }
}
struct SummaryRanges {
    intervals: BTreeMap<i32, Interval>,
}

impl SummaryRanges {
    fn new() -> Self {
        Self {
            intervals: BTreeMap::new(),
        }
    }

    fn add_num(&mut self, val: i32) {
        let mut left = val;
        let mut right = val;
        let mut to_remove = Vec::new();

        // check merge case
        for (&start, interval) in &self.intervals {
            if start <= val + 1 && interval.end >= val - 1 {
                left = left.min(interval.start);
                right = right.max(interval.end);
                to_remove.push(start);
            }
        }
        // remove interval that are merged
        for start in to_remove {
            self.intervals.remove(&start);
        }

        self.intervals.insert(left, Interval::new(left, right));
    }

    fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.intervals
            .values()
            .clone()
            .into_iter()
            .map(|itv| vec![itv.start, itv.end])
            .collect()
    }
}

fn main() {}
