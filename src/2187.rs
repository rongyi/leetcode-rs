struct Solution;

use std::i64;
impl Solution {
    pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
        let mut l = 0;
        let mut r = i64::MAX / time.len() as i64;
        while l < r {
            let mid = l + (r - l) / 2;
            let mut cur_trips = 0;
            for &t in time.iter() {
                cur_trips += mid / t as i64;
            }
            if cur_trips < total_trips as i64 {
                l = mid + 1
            } else {
                r = mid;
            }
        }

        l
    }
}

fn main() {}
