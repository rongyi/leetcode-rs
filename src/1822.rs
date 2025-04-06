#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let mut neg_acc = 0;
        for num in nums.into_iter() {
            if num == 0 {
                return 0;
            } else if num < 0 {
                neg_acc += 1;
            }
        }
        if neg_acc % 2 == 1 {
            -1
        } else {
            1
        }
    }
}
fn main() {}
