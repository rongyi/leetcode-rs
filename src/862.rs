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

            // Q: Why keep the deque increase?
            // A: If B[i] <= B[d.back()] and moreover we already know that i > d.back(), it means that compared with d.back(),
            // B[i] can help us make the subarray length shorter and sum bigger. So no need to keep d.back() in our deque.

            // More detailed on this, we always add at the LAST position
            // B[d.back] <- B[i] <- ... <- B[future id]
            // B[future id] - B[d.back()] >= k && B[d.back()] >= B[i]
            // B[future id] - B[i] >= k too

            // so no need to keep B[d.back()]
            // 也就是说我们遇到了一个更好的选择，怎么理解，与当前pos更接近的prefixsum值变小，也就
            // 意味着从这个位置到当前pos的区间sum更大了，所以前面的这个值肯定不是最优解了
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
