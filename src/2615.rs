
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
            let mut prefix_sum = 0i64;
            let sz = idx_group.len();
            let total: i64 = idx_group.iter().map(|x| *x as i64).sum();

            for (i, &v) in idx_group.iter().enumerate() {
                let post_sum = total - prefix_sum - v as i64;
                ret[v] += (v as i64) * (i as i64);
                ret[v] -= prefix_sum;
                ret[v] -= (v as i64) * ((sz - i - 1) as i64);
                ret[v] += post_sum;

                prefix_sum += v as i64;
            }
        }

        ret
    }
}

fn main() {}
