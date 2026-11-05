#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn maximum_removals(s: String, p: String, removable: Vec<i32>) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let p = p.chars().collect::<Vec<_>>();

        let is_subsequence = |k: usize| -> bool {
            let removable_set: std::collections::HashSet<i32> =
                removable[0..k].iter().cloned().collect();

            let mut i = 0;
            let mut j = 0;

            while i < s.len() && j < p.len() {
                if !removable_set.contains(&(i as i32)) && s[i] == p[j] {
                    j += 1;
                }
                i += 1;
            }

            j == p.len()
        };

        let mut left = 0;
        let mut right = removable.len();

        while left < right {
            let mid = left + (right - left) / 2;

            if is_subsequence(mid) {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        // We need to verify if 'left' calculation is correct
        // If left == removable.len() and is_subsequence(left), then we can remove all elements
        // If is_subsequence(left) is false, we need to check left - 1

        (left as i32)
            - if left > 0 && !is_subsequence(left) {
                1
            } else {
                0
            }
    }
}

fn main() {}
