struct Solution;
use std::collections::BTreeMap;

impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let sz = intervals.len();
        let mut ret: Vec<i32> = Vec::new();
        let mut cache: BTreeMap<i32, usize> = BTreeMap::new();
        for i in 0..sz {
            cache.insert(intervals[i][0], i);
        }

        for itv in intervals.iter() {
            // rust lower_bound
            match cache.range(itv[1]..).next() {
                None => ret.push(-1),
                Some((_, &idx)) => ret.push(idx as i32),
            }
        }

        ret
    }
}

fn main() {}
