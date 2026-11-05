#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        // Calculate LIS from left to right
        let mut lis = vec![1; n];
        for i in 1..n {
            for j in 0..i {
                if nums[i] > nums[j] {
                    lis[i] = lis[i].max(lis[j] + 1);
                }
            }
        }

        // Calculate LIS from right to left (effectively LDS from left to right)
        let mut lds = vec![1; n];
        for i in (0..n - 1).rev() {
            for j in (i + 1..n).rev() {
                if nums[i] > nums[j] {
                    lds[i] = lds[i].max(lds[j] + 1);
                }
            }
        }

        // Find the maximum length of a mountain subsequence
        let mut max_mountain = 0;
        for i in 0..n {
            // A valid mountain must have both increasing and decreasing parts
            if lis[i] > 1 && lds[i] > 1 {
                max_mountain = max_mountain.max(lis[i] + lds[i] - 1);
            }
        }

        // Return the minimum number of elements to remove
        (n as i32) - max_mountain
    }
}

fn main() {}
