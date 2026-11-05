struct Solution;

const P: i64 = 127;
const MOD: i64 = 1_000_000_007;

use std::{collections::HashMap, ops::Rem};
impl Solution {
    pub fn longest_dup_substring(s: String) -> String {
        let sz = s.len();
        let mut vals = vec![0i64; sz + 1];
        let mut pows = vec![1i64; sz + 1];

        for (i, cur) in s.bytes().enumerate() {
            vals[i + 1] = (vals[i] * P + cur as i64) % MOD;
            pows[i + 1] = pows[i] * P % MOD;
        }
        // binary search
        let (mut low, mut high) = (0, sz);
        let mut ret = 0;
        while low < high {
            let mid = (low + high) / 2;
            if let Some(i) = Self::search(s.as_bytes(), &vals, pows[mid], mid) {
                ret = i;
                low = mid + 1;
            } else {
                high = mid;
            }
        }

        s[ret..ret + low - 1].to_string()
    }

    fn search(s: &[u8], v: &[i64], pow: i64, len: usize) -> Option<usize> {
        let mut hash_idx = HashMap::new();
        for i in 0..=s.len() - len {
            let cur_hash = (v[i + len] - v[i] * pow).rem_euclid(MOD);
            if let Some(&j) = hash_idx.get(&cur_hash) {
                if (0..len).all(|k| s[i + k] == s[j + k]) {
                    return Some(i);
                }
            }
            hash_idx.insert(cur_hash, i);
        }

        None
    }
}

fn main() {}
