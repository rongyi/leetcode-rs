
struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut count: HashMap<i32, usize> = HashMap::new();
        for &num in nums.iter() {
            *count.entry(num).or_default() += 1;
        }
        let max_cnt = *count.values().max().unwrap();
        let mut ret: Vec<Vec<i32>> = vec![vec![]; max_cnt];

        for (&k, &v) in count.iter() {
            for r in 0..v {
                ret[r].push(k);
            }
        }

        ret
    }
}

fn main() {}
