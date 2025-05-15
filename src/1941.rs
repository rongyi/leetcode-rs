#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn are_occurrences_equal(s: String) -> bool {
        let mut cnt = vec![0; 26];
        for c in s.chars() {
            let idx = c as usize - 'a' as usize;
            cnt[idx] += 1;
        }
        let mut val = 0;
        for i in 0..26 {
            if cnt[i] == 0 {
                continue;
            }
            if val == 0 {
                val = cnt[i];
            } else {
                if cnt[i] != val {
                    return false;
                }
            }
        }

        true
    }
}

fn main() {}
