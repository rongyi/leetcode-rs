#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let stream: Vec<i32> = (1..=n).collect();
        let mut i = 0;
        let mut j = 0;
        let mut ret = Vec::new();
        while i < target.len() && j < stream.len() {
            if target[i] == stream[j] {
                ret.push("Push".to_string());
                i += 1;
                j += 1;
            } else {
                j += 1;
                ret.push("Push".to_string());
                ret.push("Pop".to_string());
            }
        }

        ret
    }
}
fn main() {}
