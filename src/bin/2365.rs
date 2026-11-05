
struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn task_scheduler_ii(tasks: Vec<i32>, space: i32) -> i64 {
        let mut prev_timestamp: HashMap<i32, i64> = HashMap::new();
        let mut timestamp = 0i64;
        let space = space as i64;
        for &t in tasks.iter() {
            if let Some(prev_ts) = prev_timestamp.get(&t) {
                let lock = *prev_ts + space;
                // wait there
                timestamp = timestamp.max(lock);
            }
            timestamp += 1;
            // after
            prev_timestamp.insert(t, timestamp);
        }

        timestamp
    }
}

fn main() {}
