
struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn tallest_billboard(rods: Vec<i32>) -> i32 {
        // left and right diff -> height
        let mut dp: HashMap<i32, i32> = HashMap::new();
        // initial case
        dp.insert(0, 0);

        for leg in rods.into_iter() {
            let mut tmp: HashMap<i32, i32> = HashMap::new();
            tmp.extend(dp.clone());

            for (&cur_diff, &v) in dp.iter() {
                // weld to left
                let left_max = (v + leg).max(*tmp.get(&(cur_diff + leg)).unwrap_or(&0));
                tmp.insert(cur_diff + leg, left_max);

                let left_max = v.max(*tmp.get(&(cur_diff - leg)).unwrap_or(&0));
                tmp.insert(cur_diff - leg, left_max);

                // weld to right
            }

            dp = tmp;
        }

        dp[&0]
    }
}

fn main() {}
