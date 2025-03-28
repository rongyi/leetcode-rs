#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut cnt = vec![0; 101];
        let mut sum = 0;
        for &num in &nums {
            if cnt[num as usize] == 1 {
                sum -= num;
            } else if cnt[num as usize] == 0 {
                sum += num;
            }
            cnt[num as usize] += 1;
        }
        sum
    }
}
fn main() {}
