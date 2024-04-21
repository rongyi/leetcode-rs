#![allow(dead_code)]

use std::collections::{BTreeMap, HashSet};

struct RangeModule {
    intervals: BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RangeModule {
    fn new() -> Self {
        Self {
            intervals: BTreeMap::new(),
        }
    }

    fn add_range(&mut self, left: i32, right: i32) {
        let mut left = left;
        let mut right = right;

        let intersects = self
            .intervals
            .range(0..=right)
            .rev()
            .take_while(|(&l, &r)| left <= r && right >= l)
            .map(|(&x, &y)| (x, y))
            .collect::<Vec<(i32, i32)>>();

        for (l, r) in intersects.into_iter() {
            right = right.max(r);
            left = left.min(l);
            self.intervals.remove(&l);
        }

        self.intervals.insert(left, right);
    }

    fn query_range(&self, left: i32, right: i32) -> bool {
        match self.intervals.range(0..=left).rev().next() {
            None => false,
            Some((&_, &r)) => r >= right,
        }
    }

    fn remove_range(&mut self, left: i32, right: i32) {
        let mut outliers = vec![];

        let intersections = self
            .intervals
            .range(0..=right)
            .rev()
            .take_while(|(&l, &r)| left < r && right > l)
            .map(|(&l, &r)| (l, r))
            .collect::<HashSet<(i32, i32)>>();

        for (l, r) in intersections.into_iter() {
            if l < left {
                outliers.push((l, left));
            }
            if r > right {
                outliers.push((right, r));
            }
            self.intervals.remove(&l);
        }

        for (l, r) in outliers.into_iter() {
            self.intervals.insert(l, r);
        }
    }
}

fn main() {
    let mut r = RangeModule::new();
    r.add_range(2, 4);
    r.add_range(3, 5);
}
