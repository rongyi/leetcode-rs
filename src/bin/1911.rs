#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut even = 0i64; // Maximum sum ending with an even-indexed element
        let mut odd = 0i64; // Maximum sum ending with an odd-indexed element

        for num in nums {
            let num = num as i64;
            let new_even = odd + num;
            let new_odd = even - num;

            even = even.max(new_even);
            odd = odd.max(new_odd);
        }

        even
    }
}

fn main() {}
