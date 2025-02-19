#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max_num = *candies.iter().max().unwrap();
        let mut ret = Vec::with_capacity(candies.len());
        for candy in candies {
            ret.push(candy + extra_candies >= max_num);
        }
        ret
    }
}

fn main() {}
