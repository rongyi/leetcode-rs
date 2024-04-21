#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn smallest_distance_pair(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let sz = nums.len();
        let mut left = 0;
        let mut right = *nums.last().unwrap() - nums[0];

        // 意思是让我们来假设当前第K个值为mid，然后我们来数一数前面有多少个数呢？
        // 如果数下来恰好是第K个，那么就对了
        // 多了，那么说明真正第K个元素比我们预估的要小(我们估大发了)，那么预估的mid要往小的方向走
        // 少了，说明真正第K个元素比我们预估的要大， 预估的值(mid)往大了走
        while left < right {
            let mid = left + (right - left) / 2;
            let mut cnt = 0;
            let mut start = 0;
            for i in 0..sz {
                while start < sz && (nums[i] - nums[start] > mid) {
                    start += 1;
                }

                cnt += i - start;
            }
            if cnt < k as usize {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        right
    }
}

fn main() {}
