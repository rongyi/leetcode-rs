#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
        let sz = arr.len();
        let mut dp = vec![i32::MAX; sz];
        let mut sum = 0;
        let mut left = 0;
        let mut result = i32::MAX;
        let mut min_len = i32::MAX;

        for right in 0..sz {
            sum += arr[right];

            while sum > target {
                sum -= arr[left];
                left += 1;
            }
            if sum == target {
                let cur_len = (right - left + 1) as i32;
                if left > 0 && dp[left - 1] != i32::MAX {
                    result = result.min(cur_len + dp[left - 1]);
                }

                min_len = min_len.min(cur_len);
                dp[right] = min_len;
            } else {
                if right > 0 {
                    dp[right] = dp[right - 1];
                }
            }
        }

        if result == i32::MAX {
            -1
        } else {
            result
        }
    }
}

fn main() {}
