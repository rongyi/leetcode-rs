#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn num_times_all_blue(flips: Vec<i32>) -> i32 {
        let mut right_most = 0;
        let mut count = 0;

        for (i, &flip) in flips.iter().enumerate() {
            right_most = right_most.max(flip);

            if right_most == (i + 1) as i32 {
                count += 1;
            }
        }

        count
    }
}

fn main() {}
