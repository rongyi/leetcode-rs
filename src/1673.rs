#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut stack = Vec::with_capacity(k);
        let n = nums.len();

        for i in 0..n {
            // We want to maintain a monotonically increasing stack
            // We can pop elements from stack if:
            // 1. Current element is smaller than the top element
            // 2. We have enough elements remaining to fill the stack to size k
            while !stack.is_empty() && stack.last().unwrap() > &nums[i] && stack.len() + (n - i) > k
            {
                stack.pop();
            }

            // If our stack is not full, add the current element
            if stack.len() < k {
                stack.push(nums[i]);
            }
        }

        stack
    }
}
fn main() {}
