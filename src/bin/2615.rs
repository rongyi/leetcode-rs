struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn distance(nums: Vec<i32>) -> Vec<i64> {
        let sz = nums.len();
        let mut ret: Vec<i64> = vec![0; sz];
        let mut index: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, v) in nums.into_iter().enumerate() {
            index.entry(v).or_default().push(i);
        }

        for idx_group in index.values() {
            let mut prefix = 0i64;
            let total: i64 = idx_group.iter().map(|x| *x as i64).sum();
            let sz = idx_group.len();
            for (i, &v) in idx_group.iter().enumerate() {
                let suffix = total - prefix - v as i64;
                ret[v] += (i as i64) * v as i64;
                ret[v] -= prefix;
                ret[v] += suffix;
                ret[v] -= (sz - i - 1) as i64 * v as i64;

                prefix += v as i64;
            }
        }

        ret
    }
}

fn main() {}
