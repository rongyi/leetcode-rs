#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let s: Vec<char> = s.chars().collect();
        let indices: Vec<usize> = indices.into_iter().map(|i| i as usize).collect();
        let mut ret: Vec<char> = vec!['a'; s.len()];

        for (i, &index) in indices.iter().enumerate() {
            ret[index] = s[i];
        }

        ret.into_iter().collect()
    }
}
fn main() {}
