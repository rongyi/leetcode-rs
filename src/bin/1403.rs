#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_subsequence(mut nums: Vec<i32>) -> Vec<i32> {
        let sum: i32 = nums.iter().sum();
        nums.sort_by(|a, b| b.cmp(a));
        let mut ret = Vec::new();

        for &num in nums.iter() {
            ret.push(num);
            if ret.iter().sum::<i32>() > sum / 2 {
                break;
            }
        }

        ret
    }
}

fn main() {}
