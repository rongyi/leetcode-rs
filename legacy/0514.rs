
struct Solution;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let ring: Vec<char> = ring.chars().collect();
        let key: Vec<char> = key.chars().collect();
        let m = ring.len();
        let n = key.len();

        // cache pos of ring
        let mut index: HashMap<char, HashSet<usize>> = HashMap::new();
        for (i, &c) in ring.iter().enumerate() {
            index.entry(c).or_insert(HashSet::new()).insert(i);
        }

        let mut dp = vec![vec![0; n]; m];

        Self::dfs(&ring, &key, &index, &mut dp, 0, 0) + n as i32
    }

    fn dfs(
        ring: &[char],
        key: &[char],
        index: &HashMap<char, HashSet<usize>>,
        dp: &mut Vec<Vec<i32>>,
        rpos: usize,
        kpos: usize,
    ) -> i32 {
        if kpos == key.len() {
            return 0;
        }
        if dp[rpos][kpos] != 0 {
            return dp[rpos][kpos];
        }
        let mut ret = i32::MAX;
        for &next_index in index.get(&key[kpos]).unwrap().iter() {
            let diff = (next_index as i32 - rpos as i32).abs();
            let step = diff.min(ring.len() as i32 - diff);

            ret = ret.min(Self::dfs(ring, key, index, dp, next_index, kpos + 1) + step);
        }

        dp[rpos][kpos] = ret;

        ret
    }
}

fn main() {}
