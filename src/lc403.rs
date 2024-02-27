struct Solution;
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        let sz = stones.len();
        let mut index = HashMap::new();
        for (i, &stone) in stones.iter().enumerate() {
            index.insert(stone, i);
        }
        let mut dp = vec![HashSet::new(); sz];
        dp[0].insert(0);

        for i in 0..sz {
            for &prev_step in &dp[i].clone() {
                for &next_step in &[prev_step - 1, prev_step, prev_step + 1] {
                    if next_step > 0 {
                        if let Some(&stone_index) = index.get(&(stones[i] + next_step)) {
                            // jump to this stone with step size of next_step
                            // so for next time, we can jump [next_step - 1, next_step, next_step + 1]
                            // this value become prev_step
                            dp[stone_index].insert(next_step);
                        }
                    }
                }
            }
        }

        !dp[sz - 1].is_empty()
    }
}

fn main() {
    println!("{}", 10i32.pow(2));
}
