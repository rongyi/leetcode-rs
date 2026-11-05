
struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut sum = 0;
        let mut modk: HashMap<i32, i32> = HashMap::new();
        modk.insert(0, -1);
        for (i, &num) in nums.iter().enumerate() {
            let i = i as i32;
            sum += num;
            sum %= k;
            if modk.contains_key(&sum) {
                if i - modk[&sum] > 1 {
                    return true;
                }
            } else {
                modk.insert(sum, i);
            }
        }
        false
    }
}

fn main() {}
