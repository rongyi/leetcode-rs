#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn xor_game(nums: Vec<i32>) -> bool {
        let mut x = 0;
        for &num in nums.iter() {
            x ^= num;
        }
        // if xor != 0, when len is even Alice win
        x == 0 || nums.len() % 2 == 0
    }
}

fn main() {}
