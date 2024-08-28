struct Solution;
use std::collections::BTreeMap;

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut durations: BTreeMap<i32, f64> = BTreeMap::new();
        let mut ret = 0;

        for (i, &pos) in position.iter().enumerate() {
            let val: f64 = (target as f64 - pos as f64) / speed[i] as f64;
            // put smaller distance point int front
            durations.insert(-pos, val);
        }
        let mut cur: f64 = 0.0;
        for (_, &v) in durations.iter() {
            // If a car takes longer to reach the target than the car in front of it, it forms a new fleet.
            // and if v <= cur, means current car will meet some car in front of it, which car?
            // we don't care, just some car in front of this car, and we dont need to distinguish this
            if v > cur {
                cur = v;
                ret += 1;
            }
        }

        ret
    }
}

fn main() {}
