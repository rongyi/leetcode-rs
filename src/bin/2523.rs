struct Solution;

use std::i32;
impl Solution {
    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        let mut ps: Vec<i32> = vec![];
        for i in left.max(2)..=right {
            // check i  is prime?
            let mut valid = true;
            for j in 2..=i {
                if j * j > i {
                    break;
                }
                if i % j == 0 {
                    valid = false;
                    break;
                }
            }
            if valid {
                ps.push(i);
            }
        }
        if ps.len() < 2 {
            return vec![-1, -1];
        }
        let mut min_gap = i32::MAX;
        let mut gap_idx = 0;
        for i in 1..ps.len() {
            let cur_gap = ps[i] - ps[i - 1];
            if cur_gap < min_gap {
                min_gap = cur_gap;
                gap_idx = i - 1;
            }
        }
        vec![ps[gap_idx], ps[gap_idx + 1]]
    }
}

fn main() {}
