struct Solution;
use std::collections::VecDeque;

impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let sz = nums.len();
        let mut prefix_sum = vec![0; sz + 1];
        for i in 0..sz {
            prefix_sum[i + 1] = prefix_sum[i] + nums[i] as i64;
        }

        let mut deque = VecDeque::new();
        let mut ret = sz as i32 + 1;

        for i in 0..=sz {
            while !deque.is_empty()
                && prefix_sum[i] - prefix_sum[*deque.front().unwrap()] >= k as i64
            {
                ret = ret.min(i as i32 - *deque.front().unwrap() as i32);
                deque.pop_front();
            }

            while !deque.is_empty() && prefix_sum[i] <= prefix_sum[*deque.back().unwrap()] {
                deque.pop_back();
            }

            deque.push_back(i);
        }

        if ret > sz as i32 {
            -1
        } else {
            ret
        }
    }
}

fn main() {}
