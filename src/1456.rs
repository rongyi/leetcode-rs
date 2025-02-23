#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let vowes = ['a', 'o', 'e', 'i', 'u'];
        let mut cur_vowels = 0;
        let mut ret = 0;

        let k = k as usize;
        // first window;
        for i in 0..k {
            if vowes.contains(&s[i]) {
                cur_vowels += 1;
            }
        }
        ret = ret.max(cur_vowels);

        for i in k..s.len() {
            if vowes.contains(&s[i - k]) {
                cur_vowels -= 1;
            }
            if vowes.contains(&s[i]) {
                cur_vowels += 1;
            }
            ret = ret.max(cur_vowels);
        }

        ret
    }
}

fn main() {}
