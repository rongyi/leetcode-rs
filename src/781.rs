#![allow(dead_code)]

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut canbemet: HashMap<i32, i32> = HashMap::new();
        let mut ret = 0;

        for &num in answers.iter() {
            if num == 0 {
                ret += 1;
                continue;
            }
            if canbemet.contains_key(&num) {
                canbemet.entry(num).and_modify(|v| *v -= 1);
                if canbemet[&num] == 0 {
                    canbemet.remove(&num);
                }
            } else {
                ret += num + 1;
                canbemet.insert(num, num);
            }
        }

        ret
    }
}

fn main() {}
