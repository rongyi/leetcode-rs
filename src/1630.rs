#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        let mut ans = Vec::new();

        for i in 0..l.len() {
            let left = l[i] as usize;
            let right = r[i] as usize;

            // Extract the subarray
            let mut subarray: Vec<i32> = nums[left..=right].to_vec();

            // Sort the subarray to check if it forms an arithmetic sequence
            subarray.sort();

            // Check if the sorted subarray forms an arithmetic sequence
            let is_arithmetic = if right - left < 1 {
                // A subarray with 0 or 1 elements is trivially arithmetic
                true
            } else {
                let diff = subarray[1] - subarray[0];
                let mut is_valid = true;

                for j in 1..subarray.len() {
                    if subarray[j] - subarray[j - 1] != diff {
                        is_valid = false;
                        break;
                    }
                }

                is_valid
            };

            ans.push(is_arithmetic);
        }

        ans
    }
}

fn main() {}
