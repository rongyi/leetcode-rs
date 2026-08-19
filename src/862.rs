struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as i64; // Use i64 to avoid overflow

        // Prefix sums: prefix[i] = sum of nums[0..i-1]
        let mut prefix = vec![0_i64; n + 1];
        for i in 0..n {
            prefix[i + 1] = prefix[i] + nums[i] as i64;
        }

        let mut deque: VecDeque<usize> = VecDeque::new();
        let mut result = i32::MAX;

        for i in 0..=n {
            // Check if we can form a valid subarray ending at i
            while !deque.is_empty() && prefix[i] - prefix[*deque.front().unwrap()] >= k {
                let start = deque.pop_front().unwrap();
                result = result.min((i - start) as i32);
            }

            // Maintain increasing order of prefix sums
            // more closer and prefixsum smaller, so the other part from i -> next_range can be more possible
            // to meet >= k condition with smaller range
            while !deque.is_empty() && prefix[i] <= prefix[*deque.back().unwrap()] {
                deque.pop_back();
            }

            deque.push_back(i);
        }

        if result == i32::MAX {
            -1
        } else {
            result
        }
    }
}

fn main() {}
