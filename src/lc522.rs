
struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn find_lu_slength(strs: Vec<String>) -> i32 {
        fn is_subsequence(s1: &str, s2: &str) -> bool {
            let s1: Vec<char> = s1.chars().collect();
            let s2: Vec<char> = s2.chars().collect();

            if s1.is_empty() {
                return true;
            }
            let m = s1.len();
            let n = s2.len();
            let mut j = 0;
            for i in 0..n {
                if s1[j] == s2[i] {
                    j += 1;
                }
                if j == m {
                    return true;
                }
            }
            false
        }
        let mut duplicate: HashSet<String> = HashSet::new();
        let mut ret = -1;
        for (i, s) in strs.iter().enumerate() {
            if duplicate.contains(s) {
                continue;
            }

            let mut ok = true;
            for j in 0..strs.len() {
                if i == j {
                    continue;
                }
                if is_subsequence(&s, &strs[j]) {
                    duplicate.insert(s.clone());
                    ok = false;
                    break;
                }
            }
            if ok {
                ret = ret.max(s.len() as i32);
            }
        }

        ret
    }
}

fn main() {}
