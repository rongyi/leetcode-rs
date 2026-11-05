struct Solution;

mod ai {
    struct Solution;

    use std::cmp::max;
    use std::collections::HashMap;


    impl Solution {
        pub fn tallest_billboard(rods: Vec<i32>) -> i32 {
            // Map: diff -> max_height_of_taller_stand
            let mut dp: HashMap<i32, i32> = HashMap::new();
            dp.insert(0, 0);

            for rod in rods {
                // Snapshot current state to avoid using the same rod multiple times
                let current_dp = dp.clone();

                for (&diff, &tall) in &current_dp {
                    // Choice 1: Add rod to the taller stand
                    let entry1 = dp.entry(diff + rod).or_insert(0);
                    *entry1 = max(*entry1, tall + rod);

                    // Choice 2: Add rod to the shorter stand
                    // The new taller height becomes max(tall, tall - diff + rod)
                    let new_diff = (diff - rod).abs();
                    let new_tall = max(tall, tall - diff + rod);
                    let entry2 = dp.entry(new_diff).or_insert(0);
                    *entry2 = max(*entry2, new_tall);
                }
            }

            // Return the height when the difference between stands is 0
            *dp.get(&0).unwrap_or(&0)
        }
    }
}

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
