#![allow(dead_code)]

struct Solution;

impl Solution {
    // sliding window
    pub fn equal_substring(s: String, t: String, mut max_cost: i32) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();
        let sz = s.len() as i32;
        let mut i = 0i32;
        let mut j = 0i32;
        while i < sz {
            max_cost -= (s[i as usize] as i8 - t[i as usize] as i8).abs() as i32;
            i += 1;
            if max_cost < 0 {
                max_cost += (s[j as usize] as i8 - t[j as usize] as i8).abs() as i32;
                j += 1;
            }
        }
        i - j
    }
}

fn main() {}
