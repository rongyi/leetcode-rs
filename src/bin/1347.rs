#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut cnt_s: Vec<i32> = vec![0; 26];
        let mut cnt_t: Vec<i32> = vec![0; 26];
        for c in s.chars() {
            let idx = c as usize - 'a' as usize;
            cnt_s[idx] += 1;
        }
        for c in t.chars() {
            let idx = c as usize - 'a' as usize;
            cnt_t[idx] += 1;
        }
        let mut diff = 0;
        for i in 0..cnt_s.len() {
            diff += (cnt_s[i] - cnt_t[i]).abs();
        }
        diff / 2
    }
}


fn main() {}
