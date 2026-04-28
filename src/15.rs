struct Solution;

use std::collections::{BTreeMap, HashSet};
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut val_idx: BTreeMap<i32, Vec<usize>> = BTreeMap::new();
        for (i, &num) in nums.iter().enumerate() {
            val_idx.entry(num).or_default().push(i);
        }
        let mut res = HashSet::new();
        for (&k1, v1) in val_idx.iter() {
            for (&k2, v2) in val_idx.iter() {
                if k2 < k1 {
                    continue;
                }
                if v1.len() == 1 && v2.len() == 1 && k1 == k2 {
                    continue;
                }
                let k3 = -(k1 + k2);
                if k3 != k1 && k3 != k2 {
                    if let Some(_v3) = val_idx.get(&k3) {
                        let mut cur = vec![k1, k2, k3];
                        cur.sort();
                        res.insert(cur);
                    }
                } else if k3 == k1 {
                    if v1.len() > 2 {
                        let mut cur = vec![k1, k2, k1];
                        cur.sort_unstable();
                        res.insert(cur);
                    }
                } else if k3 == k2 {
                    if v2.len() > 2 {
                        let mut cur = vec![k1, k2, k2];
                        cur.sort_unstable();
                        res.insert(cur);
                    }
                }
            }
        }

        res.into_iter().collect()
    }
}

fn main() {
    let input = vec![-1, 0, 1, 2, -1, -4];
    let a = Solution::three_sum(input);
    println!("{:?}", a);
}
