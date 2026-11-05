#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        let mut ret = Vec::new();

        for ck in nums.chunks(2) {
            let (freq, num) = (ck[0], ck[1]);
            ret.extend(std::iter::repeat(num).take(freq as usize));
        }

        ret
    }
}

fn main() {}
