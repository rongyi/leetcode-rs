
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut idx: HashMap<i32, usize> = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            let diff = target - *num;
            if let Some(&j) = idx.get(&diff) {
                return vec![i as i32, j as i32];
            }
            idx.insert(*num, i);
        }
        panic!("not found");
    }
}
