#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn max_power(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let sz = s.len();
        let mut max_len = 1;
        let mut prev_char = s[0];
        let mut prev_char_index = 0;

        for i in 1..sz {
            if s[i] == prev_char {
                max_len = max_len.max(i - prev_char_index + 1);
            } else {
                prev_char = s[i];
                prev_char_index = i;
            }
        }

        max_len as _
    }
}
fn main() {
    Solution::max_power("leetcode".to_string());
}
