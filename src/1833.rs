#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_ice_cream(mut costs: Vec<i32>, coins: i32) -> i32 {
        costs.sort_unstable();
        let mut count = 0;
        let mut sum = 0;
        for cost in costs {
            sum += cost;
            if sum > coins {
                break;
            }
            count += 1;
        }
        count
    }
}

fn main() {}
