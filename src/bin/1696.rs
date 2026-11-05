#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn max_result(mut nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }

        // Initialize a deque to store indices of elements in the sliding window.
        // The front of the deque will always have the maximum score possible up to that index.
        let mut dq = std::collections::VecDeque::new();
        dq.push_back(0);
        let k = k as usize;

        for i in 1..n {
            // Remove elements outside of the window [i-k, i)
            while !dq.is_empty() && *dq.front().unwrap() + k < i {
                dq.pop_front();
            }
            nums[i] += nums[*dq.front().unwrap()];

            // Remove elements with smaller scores from the back of the deque
            while !dq.is_empty() && nums[*dq.back().unwrap()] <= nums[i] {
                dq.pop_back();
            }

            // Add the current index to the deque
            dq.push_back(i);
        }

        *nums.last().unwrap()
    }
}
fn main() {}
