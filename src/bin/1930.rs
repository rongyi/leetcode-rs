#![allow(dead_code)]


struct Solution;

use std::collections::BTreeSet;
impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let sz = s.len();
        let mut start = vec![-1; 26];
        let mut end = vec![0; 26];
        let mut count = vec![0; 26];
        for i in 0..sz {
            let idx = s[i] as usize - 'a' as usize;
            if start[idx] == -1 {
                start[idx] = i as i32;
            }
            end[idx] = i;
            count[idx] += 1;
        }
        let mut ret = 0;
        for i in 0..26 {
            if count[i] < 2 {
                continue;
            }
            let uniq: BTreeSet<char> = s[start[i] as usize + 1..end[i]].iter().copied().collect();
            ret += uniq.len();
        }
        ret as _
    }
}

fn main() {}
