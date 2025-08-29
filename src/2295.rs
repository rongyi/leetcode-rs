struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn array_change(mut nums: Vec<i32>, operations: Vec<Vec<i32>>) -> Vec<i32> {
        // uniq, we can cache its' index
        let mut index_cache: HashMap<i32, usize> = HashMap::new();
        for (i, &v) in nums.iter().enumerate() {
            index_cache.insert(v, i);
        }
        for rep in operations.iter() {
            let (from_val, to_val) = (rep[0], rep[1]);
            let idx = *index_cache.get(&from_val).unwrap();
            // update index method
            index_cache.remove(&from_val);
            index_cache.insert(to_val, idx);
            nums[idx] = to_val;
        }

        nums
    }
}

fn main() {}
