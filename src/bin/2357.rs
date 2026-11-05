struct Solution;

impl Solution {
    pub fn minimum_operations(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut acc = 0;
        let mut prev = 0;
        for i in 0..nums.len() {
            if nums[i] != prev {
                acc += 1;
                prev = nums[i];
            }
        }
        acc
    }
}
fn main() {}
